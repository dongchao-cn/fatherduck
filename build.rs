use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = manifest_dir + "/libduckdb";
    println!("cargo::rustc-link-search=all={}", lib_path);
    println!("cargo::rustc-env=LD_LIBRARY_PATH={}", lib_path);
}
