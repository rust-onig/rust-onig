extern crate pkg_config;
extern crate cmake;

use std::env;

fn compile_with_cmake() {
    use cmake::Config;

    let static_link = env::var("CARGO_FEATURE_STATIC_ONIG").is_ok();

    // Builds the project in the directory located in `oniguruma`, installing it
    // into $OUT_DIR
    let mut c = Config::new("oniguruma");

    let dst = if static_link {
                  c.define("BUILD_SHARED_LIBS", "OFF")
              } else {
                  c.define("CMAKE_MACOSX_RPATH", "NO")
              }
              .build();

    let link_type = if static_link {
        "static"
    } else {
        "dylib"
    };

    println!("cargo:rustc-link-search=native={}",
             dst.join("build").display());
    println!("cargo:rustc-link-lib={}=onig", link_type);
}

pub fn main() {
    if let Ok(_) = pkg_config::find_library("oniguruma") {
        return;
    }

    compile_with_cmake();
}
