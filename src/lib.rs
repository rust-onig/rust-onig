#[macro_use]
extern crate bitflags;
extern crate libc;

use libc::c_int;
use std::ptr::null;
use std::str;
use std::fmt;

mod onig_sys;
pub mod utils;

pub struct OnigError {
    error: libc::c_int,
    error_info: Option<onig_sys::OnigErrorInfo>,
}

impl fmt::Debug for OnigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut err_buff = &mut [0 as u8; 1024];
        let len = unsafe {
            match self.error_info {
                Some(ref error_info) => {
                    onig_sys::onig_error_code_to_str(err_buff.as_mut_ptr(),
                                                     self.error,
                                                     error_info as *const onig_sys::OnigErrorInfo)
                }
                None => onig_sys::onig_error_code_to_str(err_buff.as_mut_ptr(), self.error),
            }
        };
        let err_str_slice = str::from_utf8(&err_buff[..len as usize]).unwrap();
        write!(f, "Oniguruma error: {}", err_str_slice)
    }
}

#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct OnigRegion {
    raw: *const onig_sys::OnigRegion,
}

impl OnigRegion {
    pub fn new() -> OnigRegion {
        let raw = unsafe { onig_sys::onig_region_new() };
        OnigRegion { raw: raw }
    }

    pub fn clear(&mut self) {
        unsafe {
            onig_sys::onig_region_clear(self.raw);
        }
    }

    pub fn resize(&mut self, new_size: usize) -> usize {
        unsafe { onig_sys::onig_region_resize(self.raw, new_size as c_int) as usize }
    }
}

impl Drop for OnigRegion {
    fn drop(&mut self) {
        unsafe {
            onig_sys::onig_region_free(self.raw, 1);
        }
    }
}

#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct Regex {
    raw: *const onig_sys::regex_t,
}

impl Regex {
    pub fn new(pattern: &str,
               option: onig_sys::OnigOptionType,
               syntax: *const onig_sys::OnigSyntaxTypeStruct)
               -> Result<Regex, OnigError> {

        // Convert the rust types to those required for the call to
        // `onig_new`.
        let pattern_bytes = pattern.as_bytes();
        let mut reg: *const onig_sys::regex_t = null();
        let reg_ptr = &mut reg as *mut *const onig_sys::regex_t;

        // We can use this later to get an error message to pass back
        // if regex creation fails.
        let mut error = onig_sys::OnigErrorInfo {
            enc: null(),
            par: null(),
            par_end: null(),
        };

        let err = unsafe {
            onig_sys::onig_new(reg_ptr,
                               pattern_bytes.as_ptr(),
                               pattern_bytes[pattern_bytes.len()..].as_ptr(),
                               option.bits(),
                               onig_sys::onig_encodings::UTF8,
                               syntax,
                               &mut error)
        };

        if err == 0 {
            Ok(Regex { raw: reg })
        } else {
            Err(OnigError {
                error: err,
                error_info: Some(error),
            })
        }
    }
}

impl Drop for Regex {
    fn drop(&mut self) {
        unsafe {
            onig_sys::onig_free(self.raw);
        }
    }
}

#[cfg(test)]
mod test_lib {

    use super::*;
    use super::onig_sys;

    #[test]
    fn test_region_create() {
        OnigRegion::new();
    }

    #[test]
    fn test_region_clear() {
        let mut region = OnigRegion::new();
        region.clear();
    }

    #[test]
    fn test_regex_create() {
        Regex::new(".*",
                   onig_sys::ONIG_OPTION_NONE,
                   onig_sys::onig_syntax_types::RUBY)
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "Oniguruma error: invalid character property name {foo}")]
    fn test_regex_invalid() {
        Regex::new("\\p{foo}",
                   onig_sys::ONIG_OPTION_NONE,
                   onig_sys::onig_syntax_types::RUBY)
            .unwrap();
    }
}
