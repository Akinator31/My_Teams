fn main() {
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
