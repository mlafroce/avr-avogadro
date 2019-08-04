extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("ui").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=avogadrogui");
    println!("cargo:rustc-link-lib=Qt5Widgets");
    println!("cargo:rustc-link-lib=Qt5Core");
    println!("cargo:rustc-link-lib=stdc++");
}