fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = std::path::Path::new(&manifest_dir).parent().unwrap();

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/libs");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../libs");
        return;
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/libs");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../libs");
    }
}
