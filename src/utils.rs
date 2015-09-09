use onig_sys;
use std::ffi::CStr;

pub fn onig_version() -> String {
    let raw_version = unsafe {
        CStr::from_ptr(onig_sys::onig_version())
    };

    // TODO: convert the CStr directly when that becomes stable
    String::from_utf8_lossy(raw_version.to_bytes()).to_string()
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    pub fn utils_get_version_returns_expected_version() {
        let version = onig_version();
        assert_eq!(version, "5.9.6");
    }
}
