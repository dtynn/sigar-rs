#![allow(dead_code)]

fn main() {
    #[cfg(windows)]
    win_link_libs();
}

fn win_link_libs() {
    println!("cargo:rustc-link-lib=static=ws2_32");
    println!("cargo:rustc-link-lib=static=netapi32");
    println!("cargo:rustc-link-lib=static=version");
}
