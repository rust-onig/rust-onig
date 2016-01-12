#[macro_use]
extern crate bitflags;
extern crate libc;

use libc::c_int;
use std::ptr::null;
use std::str;
use std::fmt;

pub mod onig_sys;
pub mod utils;

/// Options
///
/// This module contains all of the options bitflags which can be
/// passed to a function expecting an `OnigOptionType`. Not all
/// functions will pay attention to all options.
pub use onig_sys::onig_option_type as options;

/// Syntax Types
///
/// This module contains the list of supported syntax types. At the
/// moment it's just `RUBY`.
pub use onig_sys::onig_syntax_type as syntax_types;

/// Onig Error
///
/// This struture represents an error from the unerlying Oniguruma libray.
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

unsafe fn str_end(str: &str) -> *const u8 {
    str.as_ptr().offset(str.len() as isize)
}

fn result_to_match(res: libc::c_int) -> Option<i32> {
    if res < 0 {
        None
    } else {
        Some(res)
    }
}

impl Regex {

    /// Create a new Regex
    ///
    /// Attempts to compile a pattern into a new `Regex` instance.
    ///
    /// # Arguments
    ///
    ///  * `pattern` - The regex pattern to compile.
    ///  * `options` - The regex compilation options.
    ///  * `syntax`  - The syntax which the regex is written in.
    ///
    /// # Examples
    ///
    /// ```
    /// use onig::{options,syntax_types,Regex};
    /// let r = Regex::new("hello.*world",
    ///                    options::ONIG_OPTION_NONE,
    ///                    syntax_types::RUBY);
    /// assert!(r.is_ok());
    /// ```
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

    /// Match Str
    ///
    /// Match the regex against a string. This method will start at
    /// the beginning of the string and try and match the regex. If
    /// the regex matches then the return value is the number of
    /// characers which matched. If the regex doesn't match the return
    /// is `None`.
    ///
    /// # Arguments
    ///
    /// * `self` - The regex to match
    /// * `str` - The string slice to match against.
    /// * `options` - The regex match options.
    ///
    /// # Returns
    ///
    /// `Some(len)` if the regex matched, with `len` being the number
    /// of bytes matched. `None` if the regex doesn't match.
    ///
    /// # Examples
    ///
    /// ```
    /// use onig::Regex;
    /// use onig::{options,syntax_types};
    ///
    /// let r = Regex::new(".*",
    ///                    options::ONIG_OPTION_NONE,
    ///                    syntax_types::RUBY).unwrap();
    /// let res = r.match_str("hello", options::ONIG_OPTION_NONE);
    /// assert!(res.is_some()); // it matches
    /// assert!(res.unwrap() == 5); // 5 characters matched
    /// ```
    pub fn match_str(&self, str: &str, options: onig_sys::OnigOptionType) -> Option<i32> {
        let ret = unsafe {
            let end = str_end(str);
            onig_sys::onig_match(self.raw,
                                 str.as_ptr(),
                                 end,
                                 str.as_ptr(),
                                 0 as *const onig_sys::OnigRegion,
                                 options.bits())
        };
        result_to_match(ret)
    }

    /// Search Str
    ///
    /// Search for matches the regex in a string. This method will return the
    /// index of the first match of the regex within the string, if
    /// there is one.
    ///
    /// # Arguments
    ///
    ///  * `self` - The regex to search for
    ///  * `str` - The string to search in.
    ///  * `options` - The options for the search.
    ///
    /// # Returns
    ///
    /// `Some(pos)` if the regex matches, where `pos` is the
    /// byte-position of the start of the match. `None` if the regex
    /// doesn't match anywhere in `str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use onig::Regex;
    /// use onig::{options,syntax_types};
    ///
    /// let r = Regex::new("l{1,2}",
    ///                    options::ONIG_OPTION_NONE,
    ///                    syntax_types::RUBY).unwrap();
    /// let res = r.search_str("hello", options::ONIG_OPTION_NONE);
    /// assert!(res.is_some()); // it matches
    /// assert!(res.unwrap() == 2); // match starts at character 3
    /// ```
    pub fn search_str(&self, str: &str, options: onig_sys::OnigOptionType) -> Option<i32> {
        let ret = unsafe {
            let start = str.as_ptr();
            let end = str_end(str);
            onig_sys::onig_search(self.raw,
                                  start,
                                  end,
                                  start,
                                  end,
                                  0 as *const onig_sys::OnigRegion,
                                  options.bits())
        };
        result_to_match(ret)
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
mod tests {

    use super::*;

    fn create_regex(regex: &str) -> Regex {
        Regex::new(regex,
                   onig_sys::onig_option_type::ONIG_OPTION_NONE,
                   onig_sys::onig_syntax_type::RUBY)
            .unwrap()
    }

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
                   onig_sys::onig_option_type::ONIG_OPTION_NONE,
                   onig_sys::onig_syntax_type::RUBY)
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "Oniguruma error: invalid character property name {foo}")]
    fn test_regex_invalid() {
        create_regex("\\p{foo}");
    }

    #[test]
    fn test_simple_match() {
        let r = create_regex(".*");

        let res = r.match_str("hello wolrld", onig_sys::onig_option_type::ONIG_OPTION_NONE);

        assert!(res.is_some());
        assert!(res.unwrap() == 12);
    }

    #[test]
    fn test_failed_match() {
        let r = create_regex("foo");

        let res = r.match_str("bar", onig_sys::onig_option_type::ONIG_OPTION_NONE);
        assert!(res.is_none());
    }

    #[test]
    fn test_partial_match() {
        let r = create_regex("hello");
        let res = r.match_str("hello world", onig_sys::onig_option_type::ONIG_OPTION_NONE);

        assert!(res.is_some());
        assert!(res.unwrap() == 5);
    }

    #[test]
    fn test_simple_search() {
        let r = create_regex("hello");
        let res = r.search_str("just came to say hello :-)", options::ONIG_OPTION_NONE);

        assert!(res.is_some());
        assert!(res.unwrap() == 17);
    }
}
