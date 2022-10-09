use std::fs;
use std::io::Write;

use windows_metadata::reader::Reader;

use crate::helper::{CrateType, NamespaceMap, NamespaceSegment};

mod helper;

fn main() {
    let helper::Folders {
        workspace_root,
        crat,
        src,
    } = helper::folders("windows-app");

    let _ = fs::remove_dir_all(&src.join("Microsoft"));

    let files = helper::metadata_files(&workspace_root);
    let reader = Reader::new(&files);

    let namespace_resolver = NamespaceMap::new([
        (
            "Windows.UI.Xaml.Interop",
            Some((CrateType::Local, NamespaceSegment::new("core"))),
        ),
        ("Windows", Some((CrateType::Extern("windows"), NamespaceSegment::empty()))),
    ]);

    let deps = helper::generate_source(&reader, src.as_path(), "Microsoft", &namespace_resolver);

    let mut file = fs::File::create(&crat.join("Cargo.toml")).unwrap();

    write!(file,
           r#"[package]
name = "windows_app"
version = "0.5.0"
authors = [""]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows App SDK"
repository = "https://github.com/microsoft/windows-app-rs"
documentation = ""
readme = ".github/readme.md"

[target.i686-pc-windows-msvc.dependencies]
windows_app_i686_msvc = {{ path = "../../targets/i686_msvc", version = "0.5.0" }}

[target.x86_64-pc-windows-msvc.dependencies]
windows_app_x86_64_msvc = {{ path = "../../targets/x86_64_msvc", version = "0.5.0" }}

[target.aarch64-pc-windows-msvc.dependencies]
windows_app_aarch64_msvc = {{ path = "../../targets/aarch64_msvc", version = "0.5.0" }}

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []

[dependencies.windows]
git = "https://github.com/kaivol/windows-rs"
branch = "composable"

[features]
default = []
bootstrap = ["WindowsAppSdk_Foundation"]
interop = ["UI", "windows/Win32_System_LibraryLoader", "windows/Win32_Graphics_Gdi", "windows/Win32_Foundation", "windows/Win32_Graphics_Gdi", "windows/Win32_UI_WindowsAndMessaging"]
deprecated = ["windows/deprecated"]
implement = ["windows/implement"]
{deps}
"#)
        .unwrap();
}
