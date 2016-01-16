use std::ffi::CStr;
use onig_sys;

pub fn version() -> String {
    let raw_version = unsafe { CStr::from_ptr(onig_sys::onig_version()) };
    raw_version.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn utils_get_version_returns_expected_version() {
        let version = version();
        assert_eq!(version, "5.9.6");
    }
}
