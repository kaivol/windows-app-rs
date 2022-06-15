#[rustfmt::skip]
mod generated;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fmt};

/// Includes the Windows App Runtime Bootstrap dll in rustc-link-search.
/// This is only useful when called from a build script.
pub fn include_bootstrap_dll() {
    let path = PathBuf::from(env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"));
    let file = path.join("Microsoft.WindowsAppRuntime.Bootstrap.dll");
    File::create(&file)
        .unwrap()
        .write_all(&generated::BOOTSTRAP_DLL_BYTES)
        .unwrap();
    println!("cargo:rustc-link-search={}", path.display());
}

pub enum LinkArgTarget {
    All,
    Benches,
    Bin(&'static str),
    Bins,
    Tests,
    Examples,
}

impl LinkArgTarget {
    fn rustc_link_arg(&self, args: fmt::Arguments) {
        match self {
            LinkArgTarget::All => println!("cargo:rustc-link-arg={args}"),
            LinkArgTarget::Benches => println!("cargo:rustc-link-arg-benches={args}"),
            LinkArgTarget::Bin(bin) => println!("cargo:rustc-link-arg-{bin}={args}"),
            LinkArgTarget::Bins => println!("cargo:rustc-link-arg-bins={args}"),
            LinkArgTarget::Tests => println!("cargo:rustc-link-arg-tests={args}"),
            LinkArgTarget::Examples => println!("cargo:rustc-link-arg-examples={args}"),
        };
    }
}

pub fn embed_manifest(target: LinkArgTarget) {
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
    target.rustc_link_arg(format_args!("/MANIFEST:EMBED"));
    target.rustc_link_arg(format_args!("/MANIFESTINPUT:{}", manifest_file.display()));
    target.rustc_link_arg(format_args!("/MANIFESTUAC:NO"));
}
