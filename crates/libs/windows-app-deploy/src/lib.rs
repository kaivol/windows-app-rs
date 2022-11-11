#[rustfmt::skip]
mod generated;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fmt};

/// Copies the Windows App Runtime Bootstrap dll in rustc-link-search.
/// This is only useful when called from a build script.
pub fn include_bootstrap_dll<const N: usize>(targets: [LinkArgTarget; N]) {
    for path in LinkArgTarget::output_paths(&targets) {
        let file = path.path().join("Microsoft.WindowsAppRuntime.Bootstrap.dll");
        File::create(file).unwrap().write_all(&generated::BOOTSTRAP_DLL_BYTES).unwrap();
    }

    // All Windows App SDK DLLs with non-COM/WinRT exports must be delay-loaded to give
    // developers a chance to hook LoadLibrary with the bootstrap and get the
    // Windows App Runtime package into the dll search path.
    //
    // GNU targets already utilize delay-loaded import libraries and do not need the
    // additional linker instructions.

    LinkArgTarget::rustc_link_arg(&targets, [
        format_args!("/DELAYLOAD:microsoft.windowsappruntime.dll"),
        format_args!("/DELAYLOAD:mrm.dll"),
        format_args!("/DELAYLOAD:dwritecore.dll"),
        format_args!("/DEFAULTLIB:delayimp"),
    ]);
}

pub fn embed_manifest<const N: usize>(targets: [LinkArgTarget; N]) {
    let out_dir = env::var("OUT_DIR").expect("No `OUT_DIR` env variable set");
    let manifest_file = PathBuf::from(out_dir).join("manifest.xml");
    File::create(&manifest_file)
        .unwrap()
        .write_all(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0"></assembly>
"#
            .as_bytes(),
        )
        .unwrap();
    LinkArgTarget::rustc_link_arg(&targets, [
        format_args!("/MANIFEST:EMBED"),
        format_args!("/MANIFESTINPUT:{}", manifest_file.display()),
        format_args!("/MANIFESTUAC:NO"),
    ]);
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum LinkArgTarget {
    Benches,
    Bin(&'static str),
    Bins,
    Tests,
    Examples,
}

impl LinkArgTarget {
    fn rustc_link_arg<const N: usize>(targets: &[Self], args: [fmt::Arguments; N]) {
        for target in targets {
            for arg in args {
                match target {
                    LinkArgTarget::Benches => println!("cargo:rustc-link-arg-benches={arg}"),
                    LinkArgTarget::Bin(bin) => println!("cargo:rustc-link-arg-{bin}={arg}"),
                    LinkArgTarget::Bins => println!("cargo:rustc-link-arg-bins={arg}"),
                    LinkArgTarget::Tests => println!("cargo:rustc-link-arg-tests={arg}"),
                    LinkArgTarget::Examples => println!("cargo:rustc-link-arg-examples={arg}"),
                };
            }
        }
    }

    fn output_paths(targets: &[Self]) -> BTreeSet<OutputPath> {
        let mut paths = BTreeSet::new();
        for target in targets {
            match target {
                LinkArgTarget::Bin(_) | LinkArgTarget::Bins => paths.insert(OutputPath::Root),
                LinkArgTarget::Tests | LinkArgTarget::Benches => paths.insert(OutputPath::Deps),
                LinkArgTarget::Examples => paths.insert(OutputPath::Examples),
            };
        }
        paths
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum OutputPath {
    Root,
    Deps,
    Examples,
}

impl OutputPath {
    fn path(&self) -> PathBuf {
        let mut path = PathBuf::from(env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"));
        path.pop();
        path.pop();
        match self {
            Self::Deps => {}
            Self::Root => {
                path.pop();
            }
            Self::Examples => {
                path.pop();
                path.push("examples");
            }
        };
        path
    }
}
