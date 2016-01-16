use std::{error, fmt, str};
use std::mem::transmute;
use std::ptr::null;
use libc::c_int;
use onig_sys;
use super::{
    RegexOptions, SearchOptions, REGEX_OPTION_NONE, SEARCH_OPTION_NONE,
    Syntax, Region, Captures
};

/// This struture represents an error from the underlying Oniguruma libray.
pub struct Error {
    error: c_int,
    description: String,
}

/// This struct is a wrapper around an Oniguruma regular expression
/// pointer. This represents a compiled regex which can be used in
/// search and match operations.
#[derive(Debug)]
pub struct Regex {
    raw: onig_sys::OnigRegex,
}

impl Error {
    fn new(error: c_int, info: onig_sys::OnigErrorInfo) -> Error {
        let mut buff = &mut [0 as u8; 90];
        let len = unsafe {
            onig_sys::onig_error_code_to_str(buff.as_mut_ptr(), error, &info)
        };
        let description = str::from_utf8(&buff[..len as usize]).unwrap();
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
        write!(f, "Error({}, {})", self.error, self.description())
    }
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
        let mut reg: onig_sys::OnigRegex = null();
        let reg_ptr = &mut reg as *mut onig_sys::OnigRegex;

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
                               &onig_sys::OnigEncodingUTF8,
                               transmute(syntax),
                               &mut error)
        };

        if err == 0 {
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
    /// let res = r.match_with_options("hello", SEARCH_OPTION_NONE, None);
    /// assert!(res.is_some()); // it matches
    /// assert!(res.unwrap() == 5); // 5 characters matched
    /// ```
    pub fn match_with_options(&self,
                              str: &str,
                              options: SearchOptions,
                              region: Option<&mut Region>)
                              -> Option<usize> {
        let (beg, end) = (str.as_ptr(), str[str.len()..].as_ptr());
        let r = unsafe {
            onig_sys::onig_match(
                self.raw,
                beg, end,
                beg,
                match region {
                    Some(region) => transmute(region),
                    None => 0 as *mut onig_sys::OnigRegion,
                },
                options.bits()
            )
        };

        if r >= 0 {
            Some(r as usize)
        } else if r == -1 {
            None
        } else {
            panic!("Onig: Internal error during regex match");
        }
    }

    /// Search pattern in string
    ///
    /// Search for matches the regex in a string. This method will return the
    /// index of the first match of the regex within the string, if
    /// there is one.
    ///
    /// # Arguments
    ///
    ///  * `str` - The string to search in.
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
    /// let res = r.search_with_options("hello", SEARCH_OPTION_NONE, None);
    /// assert!(res.is_some()); // it matches
    /// assert!(res.unwrap() == 2); // match starts at character 3
    /// ```
    pub fn search_with_options(&self,
                               str: &str,
                               options: SearchOptions,
                               region: Option<&mut Region>)
                               -> Option<usize> {
        let (beg, end) = (str.as_ptr(), str[str.len()..].as_ptr());
        let r = unsafe {
            onig_sys::onig_search(
                self.raw,
                beg, end,
                beg, end,
                match region {
                    Some(region) => transmute(region),
                    None => 0 as *mut onig_sys::OnigRegion,
                },
                options.bits())
        };

        if r >= 0 {
            Some(r as usize)
        } else if r == -1 {
            None
        } else {
            panic!("Onig: Internal error during regex search");
        }
    }

    /// Returns true if and only if the regex matches the string given.
    pub fn is_match(&self, text: &str) -> bool {
        self.match_with_options(text, SEARCH_OPTION_NONE, None)
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
        self.search_with_options(text, SEARCH_OPTION_NONE, Some(&mut region))
            .map(|_| region.pos(0))
            .unwrap_or(None)
    }

    /// Returns the capture groups corresponding to the leftmost-first match
    /// in text. Capture group `0` always corresponds to the entire match.
    /// If no match is found, then `None` is returned.
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        let mut region = Region::new();
        self.search_with_options(text, SEARCH_OPTION_NONE, Some(&mut region))
            .map(|_| Captures::new(text, region))
    }

    pub fn captures_len(&self) -> usize {
        unsafe {
            onig_sys::onig_number_of_captures(self.raw) as usize
        }
    }

    pub fn capture_histories_len(&self) -> usize {
        unsafe {
            onig_sys::onig_number_of_capture_histories(self.raw) as usize
        }
    }

    pub fn names_len(&self) -> usize {
        unsafe {
            onig_sys::onig_number_of_names(self.raw) as usize
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
    use super::Regex;
    use super::super::{SEARCH_OPTION_NONE, REGEX_OPTION_NONE, Syntax, Region};

    fn create_regex(regex: &str) -> Regex {
        Regex::new(regex).unwrap()
    }

    #[test]
    fn test_regex_create() {
        Regex::with_options(
            ".*",
            REGEX_OPTION_NONE,
            Syntax::default()
        ).unwrap();

        Regex::new(r#"a \w+ word"#).unwrap();
    }

    #[test]
    #[should_panic(expected="Error(-223, invalid character property name {foo})")]
    fn test_regex_invalid() {
        create_regex("\\p{foo}");
    }

    #[test]
    fn test_failed_match() {
        let r = create_regex("foo");

        let res = r.match_with_options("bar", SEARCH_OPTION_NONE, None);
        assert!(res.is_none());
    }

    #[test]
    fn test_regex_search_with_options() {
        let mut region = Region::new();
        let regex = Regex::new("e(l+)").unwrap();

        let r = regex.search_with_options(
            "hello",
            SEARCH_OPTION_NONE,
            Some(&mut region)
        );

        assert!(region.tree().is_none());
        assert_eq!(r, Some(1));
        assert_eq!(region.len(), 2);
        let pos1 = region.pos(0).unwrap();
        let pos2 = region.pos(1).unwrap();
        assert_eq!(pos1, (1, 4));
        assert_eq!(pos2, (2, 4));
    }

    #[test]
    fn test_regex_match_with_options() {
        let mut region = Region::new();
        let regex = Regex::new("he(l+)").unwrap();

        let r = regex.match_with_options(
            "hello",
            SEARCH_OPTION_NONE,
            Some(&mut region)
        );

        assert!(region.tree().is_none());
        assert_eq!(r, Some(4));
        assert_eq!(region.len(), 2);
        let pos1 = region.pos(0).unwrap();
        let pos2 = region.pos(1).unwrap();
        assert_eq!(pos1, (0, 4));
        assert_eq!(pos2, (2, 4));
    }

    #[test]
    fn test_regex_captures() {
        let regex = Regex::new("e(l+)|(r+)").unwrap();
        let captures = regex.captures("hello").unwrap();
        assert_eq!(captures.len(), 3);
        assert_eq!(captures.is_empty(), false);
        let pos1 = captures.pos(0).unwrap();
        let pos2 = captures.pos(1).unwrap();
        let pos3 = captures.pos(2);
        assert_eq!(pos1, (1, 4));
        assert_eq!(pos2, (2, 4));
        assert_eq!(pos3, None);
        let str1 = captures.at(0).unwrap();
        let str2 = captures.at(1).unwrap();
        let str3 = captures.at(2);
        assert_eq!(str1, "ell");
        assert_eq!(str2, "ll");
        assert_eq!(str3, None);

    }

    #[test]
    fn test_regex_subcaptures() {
        let regex = Regex::new("e(l+)").unwrap();
        let captures = regex.captures("hello").unwrap();
        let caps = captures.iter().collect::<Vec<_>>();
        assert_eq!(caps[0], Some("ell"));
        assert_eq!(caps[1], Some("ll"));
        assert_eq!(caps.len(), 2);

    }

    #[test]
    fn test_regex_subcapturespos() {
        let regex = Regex::new("e(l+)").unwrap();
        let captures = regex.captures("hello").unwrap();
        let caps = captures.iter_pos().collect::<Vec<_>>();
        assert_eq!(caps[0], Some((1, 4)));
        assert_eq!(caps[1], Some((2, 4)));
        assert_eq!(caps.len(), 2);

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
    fn test_regex_search_with_region_tree() {
        use super::super::SYNTAX_OPERATOR_ATMARK_CAPTURE_HISTORY;
        let mut region = Region::new();
        let mut syntax = Syntax::ruby().clone();
        syntax.enable_operators(SYNTAX_OPERATOR_ATMARK_CAPTURE_HISTORY);

        let regex = Regex::with_options(
            "(?@a+(?@b+))|(?@c+(?@d+))",
            REGEX_OPTION_NONE,
            &syntax
        ).unwrap();

        let r = regex.search_with_options(
            "- cd aaabbb -",
            SEARCH_OPTION_NONE,
            Some(&mut region)
        );

        assert_eq!(r, Some(2));
        assert_eq!(region.len(), 5);

        let tree = region.tree().unwrap();

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.group(), 0);
        assert_eq!(tree.pos(), (2, 4));

        assert_eq!(tree[0].len(), 1);
        assert_eq!(tree[0].group(), 3);
        assert_eq!(tree[0].pos(), (2, 4));

        assert_eq!(tree[0][0].len(), 0);
        assert_eq!(tree[0][0].group(), 4);
        assert_eq!(tree[0][0].pos(), (3, 4));
    }

    #[test]
    fn test_regex_lens() {
        let regex = Regex::new("(he)(l+)(o)").unwrap();
        assert_eq!(regex.captures_len(), 3);
        assert_eq!(regex.names_len(), 0);
        assert_eq!(regex.capture_histories_len(), 0);
    }
}
