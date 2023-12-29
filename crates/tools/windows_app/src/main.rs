fn main() {
    windows_bindgen::bindgen(&["--etc", "crates/tools/windows_app/bindings.txt"]).unwrap();
}
