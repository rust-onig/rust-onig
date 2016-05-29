//! This crate provides a safe wrapper around the
//! [Oniguruma](https://github.com/kkos/oniguruma) regular expression library.
//!
//! # Examples
//!
//! ```rust
//! use onig::Regex;
//!
//! let regex = Regex::new("e(l+)").unwrap();
//! for (i, pos) in regex.captures("hello").unwrap().iter_pos().enumerate() {
//!     match pos {
//!          Some((beg, end)) =>
//!              println!("Group {} captured in position {}:{}", i, beg, end),
//!          None =>
//!              println!("Group {} is not captured", i)
//!     }
//! }
//! ```

#![feature(pattern)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate onig_sys;

mod find;
mod flags;
mod region;
mod replace;
mod names;
mod syntax;
mod tree;
mod utils;
mod pattern;

// re-export the onig types publically
pub use flags::*;
pub use names::CaptureNames;
pub use region::Region;
pub use find::{
    Captures, SubCaptures, SubCapturesPos,
    FindMatches, FindCaptures, RegexSplits, RegexSplitsN
};
pub use replace::Replacer;
pub use tree::{CaptureTreeNode, CaptureTreeNodeIter};
pub use syntax::Syntax;
pub use utils::version;

use std::{error, fmt, str};
use std::sync::Mutex;
use std::mem::transmute;
use std::ptr::{null, null_mut};
use libc::c_int;

/// This struture represents an error from the underlying Oniguruma libray.
pub struct Error {
    code: c_int,
    description: String,
}

/// This struct is a wrapper around an Oniguruma regular expression
/// pointer. This represents a compiled regex which can be used in
/// search and match operations.
#[derive(Debug)]
pub struct Regex {
    raw: onig_sys::OnigRegexMut,
}

impl Error {
    fn new(code: c_int, info: onig_sys::OnigErrorInfo) -> Error {
        let mut buff = &mut [0; onig_sys::ONIG_MAX_ERROR_MESSAGE_LEN as usize];
        let len = unsafe {
            onig_sys::onig_error_code_to_str(buff.as_mut_ptr(), code, &info)
        };
        let description = str::from_utf8(&buff[..len as usize]).unwrap();
        Error {
            code: code,
            description: description.to_owned(),
        }
    }

    /// Return Oniguruma engine error code.
    pub fn code(&self) -> i32 {
        self.code
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
        write!(f, "Error({}, {})", self.code, self.description())
    }
}

lazy_static! {
    static ref REGEX_NEW_MUTEX: Mutex<()> = Mutex::new(());
}

impl Regex {
    /// Simple regular expression constructor. Compiles a new regular
    /// expression with the default options using the ruby syntax.
    /// Once compiled, it can be used repeatedly to search in a string. If an
    /// invalid expression is given, then an error is returned.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The regex pattern to compile
    ///
    /// # Examples
    ///
    /// ```
    /// use onig::Regex;
    /// let r = Regex::new(r#"hello (\w+)"#);
    /// assert!(r.is_ok());
    /// ```
    pub fn new(pattern: &str) -> Result<Regex, Error> {
        Regex::with_options(pattern, REGEX_OPTION_NONE, Syntax::default())
    }

    /// Create a new Regex
    ///
    /// Attempts to compile a pattern into a new `Regex` instance.
    /// Once compiled, it can be used repeatedly to search in a string. If an
    /// invalid expression is given, then an error is returned.
    /// See [`onig_sys::onig_new`][regex_new] for more information.
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
    /// use onig::{Regex, Syntax, REGEX_OPTION_NONE};
    /// let r = Regex::with_options("hello.*world",
    ///                             REGEX_OPTION_NONE,
    ///                             Syntax::default());
    /// assert!(r.is_ok());
    /// ```
    ///
    /// [regex_new]: ./onig_sys/fn.onig_new.html
    pub fn with_options(pattern: &str,
                        option: RegexOptions,
                        syntax: &Syntax)
                        -> Result<Regex, Error> {
        // Convert the rust types to those required for the call to
        // `onig_new`.
        let pattern_bytes = pattern.as_bytes();
        let mut reg: onig_sys::OnigRegexMut = null_mut();
        let reg_ptr = &mut reg as *mut onig_sys::OnigRegexMut;

        // We can use this later to get an error message to pass back
        // if regex creation fails.
        let mut error = onig_sys::OnigErrorInfo {
            enc: null(),
            par: null(),
            par_end: null(),
        };


        let err = unsafe {
            // Grab a lock to make sure that `onig_new` isn't called by
            // more than one thread at a time.
            let _guard = REGEX_NEW_MUTEX.lock().unwrap();
            onig_sys::onig_new(reg_ptr,
                               pattern_bytes.as_ptr(),
                               pattern_bytes[pattern_bytes.len()..].as_ptr(),
                               option.bits(),
                               &onig_sys::OnigEncodingUTF8,
                               transmute(syntax),
                               &mut error)
        };

        if err == onig_sys::ONIG_NORMAL {
            Ok(Regex { raw: reg })
        } else {
            Err(Error::new(err, error))
        }
    }

    /// Match string
    ///
    /// Match the regex against a string. This method will start at
    /// the beginning of the string and try and match the regex. If
    /// the regex matches then the return value is the number of
    /// characers which matched. If the regex doesn't match the return
    /// is `None`.
    ///
    /// # Arguments
    ///
    /// * `str` - The string slice to match against.
    /// * `at` - The byte index in the passed slice to start matching
    /// * `options` - The regex match options.
    /// * `region` - The region for return group match range info
    ///
    /// # Returns
    ///
    /// `Some(len)` if the regex matched, with `len` being the number
    /// of bytes matched. `None` if the regex doesn't match.
    ///
    /// # Examples
    ///
    /// ```
    /// use onig::{Regex, SEARCH_OPTION_NONE};
    ///
    /// let r = Regex::new(".*").unwrap();
    /// let res = r.match_with_options("hello", 0, SEARCH_OPTION_NONE, None);
    /// assert!(res.is_some()); // it matches
    /// assert!(res.unwrap() == 5); // 5 characters matched
    /// ```
    pub fn match_with_options(&self,
                              str: &str,
                              at: usize,
                              options: SearchOptions,
                              region: Option<&mut Region>)
                              -> Option<usize> {
        let (beg, end) = (str.as_ptr(), str[str.len()..].as_ptr());
        let start = str[at..].as_ptr();
        let r = unsafe {
            onig_sys::onig_match(self.raw,
                                 beg,
                                 end,
                                 start,
                                 match region {
                                     Some(region) => transmute(region),
                                     None => 0 as *mut onig_sys::OnigRegion,
                                 },
                                 options.bits())
        };

        if r >= 0 {
            Some(r as usize)
        } else if r == onig_sys::ONIG_MISMATCH {
            None
        } else {
            panic!("Onig: Internal error during regex match");
        }
    }

    /// Search pattern in string
    ///
    /// Search for matches the regex in a string. This method will return the
    /// index of the first match of the regex within the string, if
    /// there is one. If `from` is less than `to`, then search is performed
    /// in forward order, otherwice – in backward order.
    ///
    /// # Arguments
    ///
    ///  * `str` - The string to search in.
    ///  * `from` - The byte index in the passed slice to start search
    ///  * `to` - The byte index in the passed slice to finish search
    ///  * `options` - The options for the search.
    ///  * `region` - The region for return group match range info
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
    /// use onig::{Regex, SEARCH_OPTION_NONE};
    ///
    /// let r = Regex::new("l{1,2}").unwrap();
    /// let res = r.search_with_options("hello", 0, 5, SEARCH_OPTION_NONE, None);
    /// assert!(res.is_some()); // it matches
    /// assert!(res.unwrap() == 2); // match starts at character 3
    /// ```
    pub fn search_with_options(&self,
                               str: &str,
                               from: usize,
                               to: usize,
                               options: SearchOptions,
                               region: Option<&mut Region>)
                               -> Option<usize> {
        let (beg, end) = (str.as_ptr(), str[str.len()..].as_ptr());
        let (start, range) = (str[from..].as_ptr(), str[to..].as_ptr());
        let r = unsafe {
            onig_sys::onig_search(self.raw,
                                  beg,
                                  end,
                                  start,
                                  range,
                                  match region {
                                      Some(region) => transmute(region),
                                      None => 0 as *mut onig_sys::OnigRegion,
                                  },
                                  options.bits())
        };

        if r >= 0 {
            Some(r as usize)
        } else if r == onig_sys::ONIG_MISMATCH {
            None
        } else {
            panic!("Onig: Internal error during regex search");
        }
    }

    /// Returns true if and only if the regex matches the string given.
    pub fn is_match(&self, text: &str) -> bool {
        self.match_with_options(text, 0, SEARCH_OPTION_NONE, None)
            .map(|r| r == text.len())
            .unwrap_or(false)
    }

    /// Returns the start and end byte range of the leftmost-first match in
    /// `text`. If no match exists, then `None` is returned.
    ///
    /// Note that this should only be used if you want to discover the position
    /// of the match. Testing the existence of a match is faster if you use
    /// `is_match`.
    pub fn find(&self, text: &str) -> Option<(usize, usize)> {
        let mut region = Region::new();
        self.search_with_options(text, 0, text.len(),
                                 SEARCH_OPTION_NONE, Some(&mut region))
            .map(|_| region.pos(0))
            .unwrap_or(None)
    }

    pub fn captures_len(&self) -> usize {
        unsafe { onig_sys::onig_number_of_captures(self.raw) as usize }
    }

    pub fn capture_histories_len(&self) -> usize {
        unsafe {
            onig_sys::onig_number_of_capture_histories(self.raw) as usize
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
mod tests {
    use super::*;

    #[test]
    fn test_regex_create() {
        Regex::with_options(".*",
                            REGEX_OPTION_NONE,
                            Syntax::default()).unwrap();

        Regex::new(r#"a \w+ word"#).unwrap();
    }

    #[test]
    fn test_regex_invalid() {
        let e = Regex::new("\\p{foo}").unwrap_err();
        assert_eq!(e.code(), -223);
        assert_eq!(e.description(), "invalid character property name {foo}");
    }

    #[test]
    fn test_failed_match() {
        let regex = Regex::new("foo").unwrap();
        let res = regex.match_with_options("bar", 0, SEARCH_OPTION_NONE, None);
        assert!(res.is_none());
    }

    #[test]
    fn test_regex_search_with_options() {
        let mut region = Region::new();
        let regex = Regex::new("e(l+)").unwrap();

        let r = regex.search_with_options("hello", 0, 5,
                                          SEARCH_OPTION_NONE,
                                          Some(&mut region));

        assert!(region.tree().is_none());
        assert_eq!(r, Some(1));
        assert_eq!(region.len(), 2);
        let pos1 = region.pos(0).unwrap();
        let pos2 = region.pos(1).unwrap();
        assert_eq!(pos1, (1, 4));
        assert_eq!(pos2, (2, 4));

        // test cloning here since we already have a filled region
        let cloned_region = region.clone();
        let pos1_clone = cloned_region.pos(0).unwrap();
        assert_eq!(pos1_clone, pos1);
    }

    #[test]
    fn test_regex_match_with_options() {
        let mut region = Region::new();
        let regex = Regex::new("he(l+)").unwrap();

        let r = regex.match_with_options("hello", 0,
                                         SEARCH_OPTION_NONE,
                                         Some(&mut region));

        assert!(region.tree().is_none());
        assert_eq!(r, Some(4));
        assert_eq!(region.len(), 2);
        let pos1 = region.pos(0).unwrap();
        let pos2 = region.pos(1).unwrap();
        assert_eq!(pos1, (0, 4));
        assert_eq!(pos2, (2, 4));
    }

    #[test]
    fn test_regex_is_match() {
        let regex = Regex::new("he(l+)o").unwrap();
        assert!(regex.is_match("hello"));
        assert!(!regex.is_match("hello 2.0"));
    }

    #[test]
    fn test_regex_find() {
        let regex = Regex::new("he(l+)o").unwrap();
        assert_eq!(regex.find("hey, hello!"), Some((5, 10)));
        assert_eq!(regex.find("hey, honey!"), None);
    }

    #[test]
    fn test_regex_captures_len() {
        let regex = Regex::new("(he)(l+)(o)").unwrap();
        assert_eq!(regex.captures_len(), 3);
    }
}

