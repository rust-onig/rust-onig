use std::ffi::CStr;
use onig_sys;

/// Get Version
///
/// Returns the version information for the underlying Oniguruma
/// API. This is separate from the Rust Onig and onig_sys versions.
pub fn version() -> String {
    let raw_version = unsafe { CStr::from_ptr(onig_sys::onig_version()) };
    raw_version.to_string_lossy().into_owned()
}

/// Get Copyright
///
/// Returns copyright information for the underlying Oniguruma
/// API. Rust onig is licensed seperately. For more information see
/// LICENSE.md in the source distribution.
pub fn copyright() -> String {
    let raw_copy =  unsafe { CStr::from_ptr(onig_sys::onig_copyright()) };
    raw_copy.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn utils_get_version_returns_expected_version() {
        let version = version();
        assert_eq!(version, "6.0.0");
    }

    #[test]
    pub fn utils_get_copyright_is_not_emtpy() {
        let copyright = copyright();
        assert!(copyright.len() > 0);
    }
}
