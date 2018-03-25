extern crate pkg_config;

#[cfg(not(target_env = "msvc"))]
extern crate cmake;

#[cfg(target_env = "msvc")]
#[macro_use]
extern crate duct;

use pkg_config::Config;
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

fn env_var_bool(name: &str) -> Option<bool> {
    env::var(name)
        .ok()
        .map(|s| match &s.to_string().to_lowercase()[..] {
            "0" | "no" | "false" => false,
            _ => true,
        })
}

/// # Link Type Override
///
/// Retuns the override from the environment, if any is set.
fn link_type_override() -> Option<LinkType> {
    let dynamic_env = env_var_bool("RUSTONIG_DYNAMIC_LIBONIG").map(|b| match b {
        true => LinkType::Dynamic,
        false => LinkType::Static,
    });
    let static_env = env_var_bool("RUSTONIG_STATIC_LIBONIG").map(|b| match b {
        true => LinkType::Static,
        false => LinkType::Dynamic,
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

    if env_var_bool("CARGO_FEATURE_PRINT_DEBUG").unwrap_or(false) {
        c.cflag("-DONIG_DEBUG_PARSE=1");
        c.cflag("-DONIG_DEBUG_COMPILE=1");
        c.cflag("-DONIG_DEBUG_SEARCH=1");
        c.cflag("-DONIG_DEBUG_MATCH=1");
    }

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
        .join("oniguruma")
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
    if env_var_bool("RUSTONIG_SYSTEM_LIBONIG").unwrap_or(true) {
        if let Ok(_) = Config::new().atleast_version("6.8.0").probe("oniguruma") {
            return;
        }
    }

    let link_type = link_type_override().unwrap_or(DEFAULT_LINK_TYPE);

    compile(link_type);
}
