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
             dst.join("build").to_string_lossy());
    println!("cargo:rustc-link-lib={}=onig", rustc_link_type(static_link));
}

#[cfg(target_env="msvc")]
pub fn compile(static_link: bool) {
    use std::process::Command;

    let onig_sys_dir = env::current_dir().unwrap();
    let build_dir = env::var("OUT_DIR").unwrap();
    let lib_name = if static_link { "onig_s" } else { "onig" };

    let bitness = if cfg!(target_pointer_width = "64") {
        "64"
    } else {
        "32"
    };

    // Execute the oniguruma NMAKE command for the chosen architecture.
    let cmd = format!("make_win{}.bat", bitness);
    println!("{}", cmd);
    let r = Command::new("cmd")
        .args(&["/c", &(onig_sys_dir.join(cmd).to_string_lossy())])
        .current_dir(&build_dir)
        .env_remove("MFLAGS")
        .env_remove("MAKEFLAGS")
        .output()
        .expect("error running build");

    if !r.status.success() {
        let err = String::from_utf8_lossy(&r.stderr);
        let out = String::from_utf8_lossy(&r.stdout);
        panic!("Build error:\nSTDERR:{}\nSTDOUT:{}", err, out);
    }

    println!("cargo:rustc-link-search=native={}", build_dir);
    println!("cargo:rustc-link-lib={}={}",
             rustc_link_type(static_link),
             lib_name);
}

#[cfg(not(target_env = "musl"))]
fn should_static_link() -> bool {
    env::var("CARGO_FEATURE_STATIC_ONIG").is_ok() ||
        env::var("RUSTONIG_STATIC_LIBONIG").is_ok()
}

#[cfg(target_env = "musl")]
fn should_static_link() -> bool {
    true
}

pub fn main() {
    if let Ok(_) = pkg_config::find_library("oniguruma") {
        return;
    }

    compile(should_static_link());
}
