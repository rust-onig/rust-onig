use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::metadata;

fn check_exists(file: &PathBuf) -> bool {
    if let Ok(m) = metadata(file) {
        m.is_file()
    } else {
        false
    }
}

pub fn main() {
    let out_dir_str = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);
    let out_file = out_dir.join("libonig.dylib");

    // If the file already exists then skip compiling it
    if !check_exists(&out_file) {
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
        fs::copy(".libs/libonig.dylib", out_file)
            .unwrap_or_else(|err| {
                panic!("Error copying file to output: {}", err);
            });
    }

    println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=onig");

}
