fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = std::path::Path::new(&manifest_dir).parent().unwrap().join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=myteams");
    println!("cargo:rerun-if-changed={}", lib_dir.join("libmyteams.so").display());
}
