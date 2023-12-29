use windows_app_deploy::*;

fn main() {
    std::process::Command::new("makepri").args(&[
        "new",
        "/cf",
        "priconfig.xml",
        "/pr",
        ".",
        "/of",
        "../../../target/debug/resources.pri",
        "/o",
    ])
        .spawn()
        .unwrap();

    include_bootstrap_dll([LinkArgTarget::Bins]);
    embed_manifest([LinkArgTarget::Bins]);
}
