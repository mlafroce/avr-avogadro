extern crate cmake;
use cmake::Config;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !(target.contains("android")) {
        let dst = Config::new(".").build();
        println!("cargo:rustc-link-search=native={}", dst.display());
    }
}
