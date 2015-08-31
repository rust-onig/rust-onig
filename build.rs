use std::env;
use std::process::Command;
use std::fs;

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("tar")
        .arg("zxf")
        .arg("onig-5.9.6.tar.gz")
        .status().unwrap_or_else(|err| {
            panic!("Error extracting onig tar file: {}", err);
        });
    env::set_current_dir("onig-5.9.6").unwrap();
    Command::new("./configure")
        .status().unwrap_or_else(|err| {
            panic!("Error running configure: {}", err);
        });
    Command::new("make")
        .status().unwrap_or_else(|err| {
            panic!("Error running make: {}", err);
        });
    fs::copy(".libs/libonig.dylib", format!("{}/libonig.dylib", &out_dir))
        .unwrap_or_else(|err| {
            panic!("Error copying file to output: {}", err);
        });

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=onig");

}
