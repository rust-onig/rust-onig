use libc::c_int;
use std::{error, fmt, str};
use onig_sys;

/// Onig Error
///
/// This struture represents an error from the underlying Oniguruma libray.
pub struct Error {
    error: c_int,
    description: String,
}

impl Error {
    pub fn new(error: c_int, info: Option<onig_sys::OnigErrorInfo>) -> Error {
        let mut err_buff = &mut [0 as u8; 90];
        let len = unsafe {
            match info {
                Some(ref error_info) =>
                    onig_sys::onig_error_code_to_str(
                        err_buff.as_mut_ptr(),
                        error,
                        error_info as *const onig_sys::OnigErrorInfo
                    ),
                None => onig_sys::onig_error_code_to_str(
                        err_buff.as_mut_ptr(),
                        error
                    )
            }
        };
        let description = str::from_utf8(&err_buff[..len as usize]).unwrap();
        Error { error: error, description: description.to_owned() }
    }

    /// Return Oniguruma engine error code.
    pub fn code(&self) -> isize {
        self.error as isize
    }

    /// Return error description provided by Oniguruma engine.
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oniguruma error: {}", self.description())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "onig::Error({}): {}", self.error, self.description())
    }
}
