extern crate cmake;

use std::path;

fn main() {
    let dst = cmake::Config::new("sigar")
        .static_crt(true)
        .build_target("install")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        path::Path::new(&dst).join("lib").display(),
    );

    println!("cargo:rustc-link-lib=static=sigar");
}
