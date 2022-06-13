#[rustfmt::skip]
mod generated;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Writes the Windows App Runtime Bootstrap dll to `OUT_DIR` and embeds a manifest into the executable.
/// This is only useful when called from a build script.
pub fn dll_to_output_dir() {
    let mut path = PathBuf::from(env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"));
    path.pop();
    path.pop();
    path.pop();
    path.push("Microsoft.WindowsAppRuntime.Bootstrap.dll");
    File::create(&path)
        .unwrap()
        .write_all(&generated::BOOTSTRAP_DLL_BYTES)
        .unwrap();
    println!("cargo:rerun-if-changed={}", path.display());
}

pub fn embed_manifest() {
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
    println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
        manifest_file.display()
    );
    println!("cargo:rustc-link-arg-bins=/MANIFESTUAC:NO");
}
