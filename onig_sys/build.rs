extern crate pkg_config;
extern crate cmake;

fn compile_with_cmake() {
    use cmake::Config;

    // Builds the project in the directory located in `oniguruma`, installing it
    // into $OUT_DIR
    let dst = Config::new("oniguruma")
        .define("CMAKE_MACOSX_RPATH", "NO")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("build").display());
    println!("cargo:rustc-link-lib=dylib=onig");
}

pub fn main() {
    if let Ok(_) = pkg_config::find_library("oniguruma") {
        return;
    }

    compile_with_cmake();
}
