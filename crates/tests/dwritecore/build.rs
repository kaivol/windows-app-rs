use windows_app_deploy::*;

fn main() {
    include_bootstrap_dll([LinkArgTarget::Tests]);
    embed_manifest([LinkArgTarget::Tests]);
}
