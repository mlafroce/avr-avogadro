extern crate cmake;
use cmake::Config;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let is_ci = env::var("CI").is_ok();
    if !(target.contains("android") || is_ci) {
        let dst = Config::new("ui").build();
        println!("cargo:rustc-link-search=native={}", dst.display());
    }
}
