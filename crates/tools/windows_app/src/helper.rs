use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{env, fs};

use patricia_tree::PatriciaMap;
use rayon::prelude::*;
use windows_bindgen as bindgen;
use windows_bindgen::ExternalNamespaceResolver;
use windows_metadata::reader::{File, Reader};

pub struct Folders {
    pub workspace_root: PathBuf,
    pub crat: PathBuf,
    pub src: PathBuf,
}

pub fn folders(name: &str) -> Folders {
    let workspace_root = {
        let mut temp = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        temp.pop();
        temp.pop();
        temp.pop();
        temp
    };
    let crat = workspace_root.join("crates/libs/").join(name);
    let src = crat.join("src");
    Folders {
        workspace_root,
        crat,
        src,
    }
}

pub fn metadata_files(workspace_root: &impl AsRef<Path>) -> Vec<File> {
    fs::read_dir(workspace_root.as_ref().join("winmd/"))
        .unwrap()
        .filter_map(|e| {
            e.ok()
                .filter(|e| e.file_type().unwrap().is_file())
                .map(|e| e.path().into_os_string().into_string().unwrap())
        })
        .map(|e| File::new(&e).unwrap())
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NamespaceSegment<'a> {
    inner: &'a str,
}

impl<'a> NamespaceSegment<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self {
            inner: inner.trim_start_matches('.'),
        }
    }

    pub const fn empty() -> Self {
        Self { inner: "." }
    }

    pub fn parts(&self) -> impl Iterator<Item = &str> {
        self.inner.split('.').filter(|s| !s.is_empty())
    }

    pub fn join_into<'b>(&self, separator: &'b str, into: &mut String) {
        let mut iter = self.parts();
        if let Some(s) = iter.next() {
            into.push_str(s);
        }
        for s in iter {
            into.push_str(separator);
            into.push_str(s);
        }
    }

    pub fn join_with_suffix_into<'b>(&self, separator: &'b str, into: &mut String) {
        let iter = self.parts();
        for s in iter {
            into.push_str(s);
            into.push_str(separator);
        }
    }
}

#[derive(Copy, Clone)]
pub enum CrateType<'a> {
    Extern(&'a str),
    Local,
}

pub struct NamespaceMap<'a>(PatriciaMap<Option<(CrateType<'a>, NamespaceSegment<'a>)>>);

impl<'a> NamespaceMap<'a> {
    pub fn new<const N: usize>(
        namespaces: [(&'a str, Option<(CrateType<'a>, NamespaceSegment<'a>)>); N],
    ) -> Self {
        let mut map = PatriciaMap::new();
        for (namespace, crat) in namespaces {
            map.insert(namespace, crat);
        }
        Self(map)
    }

    fn resolve(
        &'a self,
        namespace: &'a str,
    ) -> Option<(CrateType, NamespaceSegment, NamespaceSegment)> {
        match self.0.get_longest_common_prefix(namespace) {
            None => None,
            Some((_, None)) => None,
            Some((key, Some((crat, path)))) => {
                Some((*crat, *path, NamespaceSegment::new(&namespace[key.len()..])))
            }
        }
    }
}
impl ExternalNamespaceResolver for NamespaceMap<'_> {
    fn resolve<'a>(&'a self, namespace: &'a str) -> Option<String> {
        self.resolve(namespace).map(|(crat, path, rest)| {
            let mut res = match crat {
                CrateType::Extern(crat) => format!("::{crat}::"),
                CrateType::Local => "crate::".to_string(),
            };
            path.join_with_suffix_into("::", &mut res);
            rest.join_with_suffix_into("::", &mut res);
            res
        })
    }

    fn is_external(&self, namespace: &str) -> bool {
        self.resolve(namespace).is_some()
    }
}

pub fn generate_source(
    reader: &Reader,
    output: &Path,
    generate_root: &str,
    namespace_resolver: &NamespaceMap,
) -> String {
    let trees = reader.tree(generate_root, &[]).unwrap();
    let trees = trees.flatten();

    trees
        .par_iter()
        .filter_map(|tree| {
            println!("{}", tree.namespace);
            let output = output.join(tree.namespace.replace('.', "/"));
            fs::create_dir_all(&output).unwrap();

            let mut gen = windows_bindgen::Gen::new(reader);
            gen.namespace = tree.namespace;
            gen.cfg = true;
            gen.doc = true;
            gen.extern_namespaces = Some(namespace_resolver);
            let mut tokens = bindgen::namespace(&gen, tree);
            tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
            tool_lib::rustfmt(tree.namespace, &mut tokens);
            fs::write(output.join("mod.rs"), tokens).unwrap();

            let mut tokens = bindgen::namespace_impl(&gen, tree);
            tool_lib::rustfmt(tree.namespace, &mut tokens);
            fs::write(output.join("impl.rs"), tokens).unwrap();

            let feature = tree.namespace.split_once('.')?.1.replace('.', "_");

            let dependencies = reader
                .namespace_types(tree.namespace)
                .flat_map(|t| reader.type_def_methods(t))
                .flat_map(|t| {
                    let sig = reader.method_def_signature(t, &[]);
                    reader.signature_cfg(&sig).types.keys().cloned().collect::<HashSet<_>>()
                })
                .collect::<HashSet<_>>();

            let mut sub_features = Vec::<String>::new();

            if let Some((parent, _)) = feature.rsplit_once('_') {
                sub_features.push(parent.to_owned());
            }

            sub_features.extend(dependencies.iter().filter_map(|f| {
                const EMPTY: NamespaceSegment = NamespaceSegment::empty();
                match namespace_resolver.resolve(f) {
                    Some((CrateType::Extern("windows"), EMPTY, n2)) => {
                        let mut res = "windows/".to_string();
                        n2.join_into("_", &mut res);
                        Some(res)
                    }
                    _ => None,
                }
            }));

            let mut features = sub_features.iter().map(|f| format!(r#""{f}""#)).collect::<Vec<_>>();
            features.sort();
            let features = features.join(",");

            Some(format!("{feature} = [{features}]"))
        })
        .collect::<Vec<_>>()
        .join("\n")
}
