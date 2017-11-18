extern crate pkg_config;

#[cfg(not(target_env = "msvc"))]
extern crate cmake;

#[cfg(target_env = "msvc")]
#[macro_use]
extern crate duct;

use std::env;
use std::fmt;

/// # Link Type Enumeration
///
/// Holds the different types of linking we support in this
/// script. Used to keep track of what the default link type is and
/// what override has been specified, if any, in the environment.
enum LinkType {
    /// Static linking. This corresponds to the `static` type in Cargo.
    Static,
    /// Dynamic linking. This corresponds to the `dylib` type in Cargo.
    Dynamic,
}

impl fmt::Display for LinkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &LinkType::Static => "static",
                &LinkType::Dynamic => "dylib",
            }
        )
    }
}

/// # Link Type Override
///
/// Retuns the override from the environment, if any is set.
fn link_type_override() -> Option<LinkType> {
    let dynamic_env = env::var("RUSTONIG_DYNAMIC_LIBONIG").ok().map(|s| {
        match &s.to_string().to_lowercase()[..] {
            "0" | "no" | "false" => LinkType::Static,
            _ => LinkType::Dynamic,
        }
    });
    let static_env = env::var("RUSTONIG_STATIC_LIBONIG").ok().map(|s| {
        match &s.to_string().to_lowercase()[..] {
            "0" | "no" | "false" => LinkType::Dynamic,
            _ => LinkType::Static,
        }
    });

    dynamic_env.or(static_env)
}

/// Default to static linking
const DEFAULT_LINK_TYPE: LinkType = LinkType::Static;

#[cfg(not(target_env = "msvc"))]
fn compile(link_type: LinkType) {
    use cmake::Config;

    // Builds the project in the directory located in `oniguruma`, installing it
    // into $OUT_DIR
    let mut c = Config::new("oniguruma");

    let dst = match link_type {
        LinkType::Static => c.define("BUILD_SHARED_LIBS", "OFF"),
        LinkType::Dynamic => c.define("CMAKE_MACOSX_RPATH", "NO"),
    }.build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("build").to_string_lossy()
    );
    println!("cargo:rustc-link-lib={}=onig", link_type);
}

#[cfg(target_env = "msvc")]
fn compile(link_type: LinkType) {

    let onig_sys_dir = env::current_dir().unwrap();
    let build_dir = env::var("OUT_DIR").unwrap();
    let lib_name = match link_type {
        LinkType::Static => "onig_s",
        LinkType::Dynamic => "onig",
    };

    let bitness = if cfg!(target_pointer_width = "64") {
        "64"
    } else {
        "32"
    };

    // Execute the oniguruma NMAKE command for the chosen architecture.
    let cmd = onig_sys_dir
        .join(format!("make_win{}.bat", bitness))
        .to_string_lossy()
        .into_owned();
    cmd!("cmd", "/c", cmd)
        .dir(&build_dir)
        .env_remove("MFLAGS")
        .env_remove("MAKEFLAGS")
        .read()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", build_dir);
    println!("cargo:rustc-link-lib={}={}", link_type, lib_name);
}

pub fn main() {
    if let Ok(_) = pkg_config::find_library("oniguruma") {
        return;
    }

    let link_type = link_type_override().unwrap_or(DEFAULT_LINK_TYPE);

    compile(link_type);
}
