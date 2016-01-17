extern crate pkg_config;

use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;

#[cfg(target_os = "macos")]
static LIB_NAME: &'static str = "libonig.dylib";
#[cfg(not(target_os = "macos"))]
static LIB_NAME: &'static str = "libonig.so";

fn compile_with_make() {
    let out_dir_str = env!("OUT_DIR");
    let out_dir = Path::new(out_dir_str);
    let onig_tar_out_dir = Path::new("./onig-5.9.6/");
    let out_file = out_dir.join(LIB_NAME);

    // If the file already exists then skip compiling it
    if !out_file.exists() {
        if onig_tar_out_dir.exists() {
            fs::remove_dir_all(onig_tar_out_dir).unwrap_or_else(|err| {
                panic!("Could not remove tar output directory: {}", err);
            });
        }
        Command::new("tar")
            .arg("zxf")
            .arg("onig-5.9.6.tar.gz")
            .status().unwrap_or_else(|err| {
                panic!("Error extracting onig tar file: {}", err);
            });
        env::set_current_dir(onig_tar_out_dir).unwrap();
        Command::new("./configure")
            .arg(format!("--prefix={}", out_dir_str))
            .status().unwrap_or_else(|err| {
                panic!("Error running configure: {}", err);
            });
        Command::new("make")
            .status().unwrap_or_else(|err| {
                panic!("Error running make: {}", err);
            });
        Command::new("make")
            .arg("install")
            .status().unwrap_or_else(|err| {
                panic!("Error running make install: {}", err);
            });
        fs::copy(fs::canonicalize(Path::new(".libs").join(LIB_NAME)).unwrap(), out_file)
            .unwrap_or_else(|err| {
                panic!("Error copying file to output: {}", err);
            });
    }

    println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=onig");

}

pub fn main() {
    if let Ok(_) = pkg_config::find_library("oniguruma") {
        return;
    }

    compile_with_make();
}
