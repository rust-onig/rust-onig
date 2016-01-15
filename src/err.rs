use libc;
use std::error::Error;
use std::{fmt, str};
use onig_sys;

/// Onig Error
///
/// This struture represents an error from the underlying Oniguruma libray.
pub struct OnigError {
    error: libc::c_int,
    description: String,
}

impl OnigError {
    pub fn new(error: libc::c_int, error_info: &onig_sys::OnigErrorInfo) -> Self {
        let mut err_buff = &mut [0 as u8; 1024];
        let len = unsafe {
            onig_sys::onig_error_code_to_str(err_buff.as_mut_ptr(),
                                             error,
                                             error_info as *const onig_sys::OnigErrorInfo)
        };
        OnigError {
            error: error,
            description: str::from_utf8(&err_buff[..len as usize]).unwrap().to_string(),
        }
    }
}

impl Error for OnigError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for OnigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oniguruma error: {}", self.description())
    }
}

impl fmt::Debug for OnigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oniguruma error({}): {}", self.error, self.description())
    }
}
