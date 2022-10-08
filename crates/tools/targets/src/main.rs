use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use quote::*;

fn main() {
    let workspace_root = {
        let mut temp = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        temp.pop();
        temp.pop();
        temp.pop();
        temp
    };
    let libs = workspace_root.join("crates/libs/");

    let windows_app_deploy = libs.join("windows-app-deploy/");
    gen_bootstrap(&windows_app_deploy);
}

fn gen_bootstrap(output: &std::path::Path) {
    let x86 = include_bytes!("../bootstrap/x86/Microsoft.WindowsAppRuntime.Bootstrap.dll");
    let x86_length = x86.len();
    let x64 = include_bytes!("../bootstrap/x64/Microsoft.WindowsAppRuntime.Bootstrap.dll");
    let x64_length = x64.len();
    let arm64 = include_bytes!("../bootstrap/arm64/Microsoft.WindowsAppRuntime.Bootstrap.dll");
    let arm64_length = arm64.len();

    let tokens = quote! {
        #[cfg(target_arch = "x86")]
        pub static BOOTSTRAP_DLL_BYTES:[u8;#x86_length] = [ #(#x86,)* ];
        #[cfg(target_arch = "x86_64")]
        pub static BOOTSTRAP_DLL_BYTES:[u8;#x64_length] = [ #(#x64,)* ];
        #[cfg(target_arch = "arm64")]
        pub static BOOTSTRAP_DLL_BYTES:[u8;#arm64_length] = [ #(#arm64,)* ];
    };

    let output = output.join("src/generated.rs");
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&output)
        .unwrap()
        .write_all(tokens.to_string().as_bytes())
        .unwrap();
}
