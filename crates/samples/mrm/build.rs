use windows_app_deploy::*;

fn main() {
    include_bootstrap_dll([LinkArgTarget::Bins]);
    embed_manifest([LinkArgTarget::Bins]);
}
