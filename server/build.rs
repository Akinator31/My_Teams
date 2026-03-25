fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = std::path::Path::new(&manifest_dir).parent().unwrap();
    let lib_dir = project_root.join("libs");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=dylib=myteams_macos");
        println!("cargo:rerun-if-changed={}", lib_dir.join("libmyteams_macos.so").display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/libs");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../libs");
        return;
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=dylib=myteams_linux");
        println!("cargo:rerun-if-changed={}", lib_dir.join("libmyteams_linux.so").display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/libs");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../libs");
    }
}
