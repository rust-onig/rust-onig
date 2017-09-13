extern crate pkg_config;

#[cfg(not(target_env="msvc"))]
extern crate cmake;

use std::env;

fn rustc_link_type(static_link: bool) -> &'static str {
    if static_link { "static" } else { "dylib" }
}


#[cfg(not(target_env="msvc"))]
fn compile(static_link: bool) {
    use cmake::Config;

    // Builds the project in the directory located in `oniguruma`, installing it
    // into $OUT_DIR
    let mut c = Config::new("oniguruma");

    let dst = if static_link {
            c.define("BUILD_SHARED_LIBS", "OFF")
        } else {
            c.define("CMAKE_MACOSX_RPATH", "NO")
        }
        .build();

    println!("cargo:rustc-link-search=native={}",
             dst.join("build").display());
    println!("cargo:rustc-link-lib={}=onig", rustc_link_type(static_link));
}

#[cfg(target_env="msvc")]
pub fn compile(static_link: bool) {
    use std::process::Command;

    let onig_dir = env::current_dir().unwrap().join("oniguruma");
    let build_dir = onig_dir.join("src");
    let lib_name = if static_link { "onig_s" } else { "onig" };

    let bitness = if cfg!(target_pointer_width = "64") {
        "64"
    } else {
        "32"
    };

    let clean = Command::new("nmake")
        .args(&["-f", "Makefile.windows", "clean"])
        .current_dir(onig_dir.join("src"))
        .env_remove("MFLAGS")
        .env_remove("MAKEFLAGS")
        .status()
        .expect("error cleaning repository");

    if !clean.success() {
        panic!("Build error: clean returned '{}'", clean);
    }

    // Execute the oniguruma NMAKE command for the chosen architecture.
    let r = Command::new("cmd")
        .args(&["/c", &format!("make_win{}.bat", bitness)])
        .current_dir(&onig_dir)
        .env_remove("MFLAGS")
        .env_remove("MAKEFLAGS")
        .output()
        .expect("error running build");

    if !r.status.success() {
        let err = String::from_utf8_lossy(&r.stderr);
        let out = String::from_utf8_lossy(&r.stdout);
        panic!("Build error:\nSTDERR:{}\nSTDOUT:{}", err, out);
    }

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib={}={}",
             rustc_link_type(static_link),
             lib_name);
}

pub fn main() {
    if let Ok(_) = pkg_config::find_library("oniguruma") {
        return;
    }

    let static_link = env::var("CARGO_FEATURE_STATIC_ONIG").is_ok() ||
                      env::var("RUSTONIG_STATIC_LIBONIG").is_ok();

    compile(static_link);
}
