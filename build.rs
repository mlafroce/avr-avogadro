extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("ui").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
}
