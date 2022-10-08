use windows_app_deploy::*;

fn main() {
    include_bootstrap_dll();
    embed_manifest(LinkArgTarget::Tests);
}
