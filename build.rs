#![allow(dead_code)]

fn main() {
    #[cfg(windows)]
    win_link_libs();
}

fn win_link_libs() {
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=netapi32");
    println!("cargo:rustc-link-lib=version");
}
