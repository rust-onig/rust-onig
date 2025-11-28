use crate::{Captures, EncodedChars, Error, Regex, RegexOptions, Region, SearchOptions};

use std::os::raw::c_int;
use std::ptr::null_mut;

/// Defines the search priority when multiple regexes could match at the same position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegSetLead {
    /// Return the match that occurs first in the text (position priority)
    Position,
    /// Same results as Position I think but slower
    Regex,
    /// Return the first regex in your regset that matches, regardless of position
    PriorityToRegexOrder,
}

impl RegSetLead {
    fn to_onig_lead(self) -> onig_sys::OnigRegSetLead {
        match self {
            RegSetLead::Position => onig_sys::OnigRegSetLead_ONIG_REGSET_POSITION_LEAD,
            RegSetLead::Regex => onig_sys::OnigRegSetLead_ONIG_REGSET_REGEX_LEAD,
            RegSetLead::PriorityToRegexOrder => {
                onig_sys::OnigRegSetLead_ONIG_REGSET_PRIORITY_TO_REGEX_ORDER
            }
        }
    }
}

/// A `RegSet` allows you to compile multiple regular expressions and search
/// for any of them in a single pass through the text. This is more efficient
/// than searching with each regex individually but `RegSet` has to own them.
#[derive(Debug)]
pub struct RegSet {
    raw: *mut onig_sys::OnigRegSet,
    options: RegexOptions,
}

unsafe impl Send for RegSet {}
unsafe impl Sync for RegSet {}

impl RegSet {
    /// Create a new RegSet from a slice of pattern strings
    ///
    /// All patterns will be compiled with default Regex options.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::RegSet;
    ///
    /// let set = RegSet::new(&[r"\d+", r"[a-z]+", r"[A-Z]+"]).unwrap();
    /// ```
    pub fn new(patterns: &[&str]) -> Result<RegSet, Error> {
        Self::with_options(patterns, RegexOptions::REGEX_OPTION_NONE)
    }

    /// Create a new RegSet from a slice of pattern strings with specified options
    ///
    /// All patterns will be compiled with the specified Regex options.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::{RegSet, RegexOptions};
    ///
    /// let set = RegSet::with_options(&[r"\d+", r"[a-z]+"], RegexOptions::REGEX_OPTION_CAPTURE_GROUP).unwrap();
    /// ```
    pub fn with_options(patterns: &[&str], options: RegexOptions) -> Result<RegSet, Error> {
        let mut regset = Self::empty_with_options(options)?;

        for pat in patterns {
            regset.add_pattern(pat)?;
        }

        Ok(regset)
    }

    /// Create an empty RegSet
    ///
    /// Creates a new empty RegSet that contains no regular expressions.
    /// Patterns can be added later using the `add_pattern` method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::RegSet;
    ///
    /// let empty_set = RegSet::empty().unwrap();
    /// assert_eq!(empty_set.len(), 0);
    /// assert!(empty_set.is_empty());
    /// ```
    pub fn empty() -> Result<RegSet, Error> {
        Self::empty_with_options(RegexOptions::REGEX_OPTION_NONE)
    }

    /// Create an empty RegSet with specified options
    ///
    /// Creates a new empty RegSet that contains no regular expressions.
    /// Patterns added later will use the specified options.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::{RegSet, RegexOptions};
    ///
    /// let empty_set = RegSet::empty_with_options(RegexOptions::REGEX_OPTION_CAPTURE_GROUP).unwrap();
    /// assert_eq!(empty_set.len(), 0);
    /// assert!(empty_set.is_empty());
    /// ```
    pub fn empty_with_options(options: RegexOptions) -> Result<RegSet, Error> {
        let mut raw_set: *mut onig_sys::OnigRegSet = null_mut();
        let raw_set_ptr = &mut raw_set as *mut *mut onig_sys::OnigRegSet;

        let err = unsafe { onig_sys::onig_regset_new(raw_set_ptr, 0, null_mut()) };

        if err != onig_sys::ONIG_NORMAL as i32 {
            return Err(Error::from_code(err));
        }

        if raw_set.is_null() {
            return Err(Error::custom("Failed to create RegSet"));
        }

        Ok(RegSet {
            raw: raw_set,
            options,
        })
    }

    /// Adds a new compiled regex pattern to the end of the RegSet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::RegSet;
    ///
    /// let mut set = RegSet::empty().unwrap();
    /// let idx = set.add_pattern(r"\d+").unwrap();
    /// assert_eq!(idx, 0);
    /// assert_eq!(set.len(), 1);
    ///
    /// let idx2 = set.add_pattern(r"[a-z]+").unwrap();
    /// assert_eq!(idx2, 1);
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn add_pattern(&mut self, pattern: &str) -> Result<usize, Error> {
        // Compile the new regex using stored options
        let new_regex = Regex::with_options(pattern, self.options, crate::Syntax::default())?;

        // Get the current length (this will be the index of the new pattern)
        let new_index = self.len();

        // Add the regex to the regset
        let err = unsafe { onig_sys::onig_regset_add(self.raw, new_regex.as_raw()) };

        if err != onig_sys::ONIG_NORMAL as i32 {
            return Err(Error::from_code(err));
        }

        // Transfer ownership of the regex to the regset
        std::mem::forget(new_regex);

        Ok(new_index)
    }

    /// Replace a regex pattern at the specified index
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::RegSet;
    ///
    /// let mut set = RegSet::new(&[r"\d+", r"[a-z]+"]).unwrap();
    /// set.replace_pattern(0, r"[A-Z]+").unwrap();
    ///
    /// assert!(set.find("123").is_none());
    /// assert!(set.find("ABC").is_some());
    /// ```
    pub fn replace_pattern(&mut self, index: usize, pattern: &str) -> Result<(), Error> {
        let regset_len = self.len();
        if index >= regset_len {
            return Err(Error::custom(format!(
                "Index {} is out of bounds for RegSet with {} regexes",
                index, regset_len
            )));
        }

        let new_regex = Regex::with_options(pattern, self.options, crate::Syntax::default())?;

        // Replace the regex in the regset
        let err =
            unsafe { onig_sys::onig_regset_replace(self.raw, index as c_int, new_regex.as_raw()) };

        if err != onig_sys::ONIG_NORMAL as i32 {
            return Err(Error::from_code(err));
        }

        // Transfer ownership of the regex to the regset
        std::mem::forget(new_regex);

        Ok(())
    }

    /// Returns the number of regexes in the set
    pub fn len(&self) -> usize {
        unsafe { onig_sys::onig_regset_number_of_regex(self.raw) as usize }
    }

    /// Returns true if the RegSet contains no regexes
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Find the first match of any regex in the set
    ///
    /// Returns a tuple of `(regex_index, match_position)` if a match is found,
    /// or `None` if no match is found.
    /// # Examples
    ///
    /// ```rust
    /// use onig::RegSet;
    ///
    /// let set = RegSet::new(&[r"\d+", r"[a-z]+"]).unwrap();
    /// if let Some((regex_index, pos)) = set.find("hello123") {
    ///     println!("Regex {} matched at position {}", regex_index, pos);
    /// }
    /// ```
    pub fn find(&self, text: &str) -> Option<(usize, usize)> {
        self.find_with_options(
            text,
            RegSetLead::Position,
            SearchOptions::SEARCH_OPTION_NONE,
        )
    }

    /// Find the first match of any regex in the set with custom options
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::{RegSet, RegSetLead, SearchOptions};
    ///
    /// let set = RegSet::new(&[r"\d+", r"[a-z]+"]).unwrap();
    /// if let Some((regex_index, pos)) = set.find_with_options(
    ///     "hello123",
    ///     RegSetLead::Regex,
    ///     SearchOptions::SEARCH_OPTION_NONE
    /// ) {
    ///     println!("Regex {} matched at position {}", regex_index, pos);
    /// }
    /// ```
    pub fn find_with_options(
        &self,
        text: &str,
        lead: RegSetLead,
        options: SearchOptions,
    ) -> Option<(usize, usize)> {
        self.search_with_encoding(text, 0, text.len(), lead, options)
    }

    /// Find the first match of any regex in the set with full capture group information
    ///
    /// Returns a tuple of `(regex_index, captures)` if a match is found,
    /// or `None` if no match is found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::RegSet;
    ///
    /// let set = RegSet::new(&[r"(\d+)", r"([a-z]+)"]).unwrap();
    /// if let Some((regex_index, captures)) = set.captures("hello123") {
    ///     println!("Regex {} matched", regex_index);
    ///     println!("Full match: {:?}", captures.at(0));
    ///     println!("First capture group: {:?}", captures.at(1));
    /// }
    /// ```
    pub fn captures<'t>(&self, text: &'t str) -> Option<(usize, Captures<'t>)> {
        self.captures_with_encoding(
            text,
            0,
            text.len(),
            RegSetLead::Position,
            SearchOptions::SEARCH_OPTION_NONE,
        )
    }

    /// Find the first match with full capture group information and encoding support
    ///
    /// Returns a tuple of `(regex_index, captures)` if a match is found,
    /// or `None` if no match is found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use onig::{RegSet, RegSetLead, SearchOptions, EncodedBytes};
    ///
    /// let set = RegSet::new(&[r"(\d+)", r"([a-z]+)"]).unwrap();
    /// if let Some((regex_index, captures)) = set.captures_with_encoding(
    ///     "hello123",
    ///     0,
    ///     8,
    ///     RegSetLead::Position,
    ///     SearchOptions::SEARCH_OPTION_NONE
    /// ) {
    ///     println!("Regex {} matched", regex_index);
    ///     println!("Full match: {:?}", captures.at(0));
    ///     println!("First capture group: {:?}", captures.at(1));
    /// }
    /// ```
    pub fn captures_with_encoding<'t, T>(
        &self,
        chars: T,
        from: usize,
        to: usize,
        lead: RegSetLead,
        options: SearchOptions,
    ) -> Option<(usize, Captures<'t>)>
    where
        T: EncodedChars,
    {
        if let Some((regex_index, match_pos)) =
            self.do_search_with_encoding(&chars, from, to, lead, options)
        {
            let region_ptr =
                unsafe { onig_sys::onig_regset_get_region(self.raw, regex_index as c_int) };

            if !region_ptr.is_null() {
                // Pre-allocate region with reasonable capacity
                // Most regexes have < 10 capture groups and it's not worth adding an option for that
                // for RegSet
                let mut region = Region::with_capacity(10);
                unsafe {
                    onig_sys::onig_region_copy(&mut region.raw, region_ptr);
                }

                // Extract text from chars only when we need it for Captures
                // SAFETY: Assumes UTF-8 encoding (safe for current EncodedChars impls)
                let text = unsafe {
                    let start_ptr = chars.start_ptr();
                    let len = chars.len();
                    let slice = std::slice::from_raw_parts(start_ptr, len);
                    std::str::from_utf8_unchecked(slice)
                };

                let captures = Captures::new(text, region, match_pos);
                return Some((regex_index, captures));
            }
        }
        None
    }

    fn do_search_with_encoding<T>(
        &self,
        chars: &T,
        from: usize,
        to: usize,
        lead: RegSetLead,
        options: SearchOptions,
    ) -> Option<(usize, usize)>
    where
        T: EncodedChars,
    {
        if from > chars.len() || to > chars.len() || from > to {
            return None;
        }

        let mut rmatch_pos: c_int = 0;
        let rmatch_pos_ptr = &mut rmatch_pos as *mut c_int;

        let (beg, end) = (chars.start_ptr(), chars.limit_ptr());

        let result = unsafe {
            let start = beg.add(from);
            let range = beg.add(to);

            onig_sys::onig_regset_search(
                self.raw,
                beg,
                end,
                start,
                range,
                lead.to_onig_lead(),
                options.bits(),
                rmatch_pos_ptr,
            )
        };

        if result >= 0 {
            Some((result as usize, rmatch_pos as usize))
        } else {
            None
        }
    }

    fn search_with_encoding<T>(
        &self,
        chars: T,
        from: usize,
        to: usize,
        lead: RegSetLead,
        options: SearchOptions,
    ) -> Option<(usize, usize)>
    where
        T: EncodedChars,
    {
        self.do_search_with_encoding(&chars, from, to, lead, options)
    }
}

impl Drop for RegSet {
    fn drop(&mut self) {
        unsafe {
            onig_sys::onig_regset_free(self.raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regset_empty_patterns() {
        let set = RegSet::new(&[]).unwrap();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_regset_new() {
        let set = RegSet::new(&[r"\d+"]).unwrap();
        assert_eq!(set.len(), 1);
        assert!(!set.is_empty());
    }

    #[test]
    fn test_regset_find_with_options() {
        let set = RegSet::new(&[r"\d+", r"[a-z]+"]).unwrap();

        let result = set.find_with_options(
            "hello123",
            RegSetLead::Position,
            SearchOptions::SEARCH_OPTION_NONE,
        );
        assert!(result.is_some());

        let result = set.find_with_options(
            "hello123",
            RegSetLead::Regex,
            SearchOptions::SEARCH_OPTION_NONE,
        );
        assert!(result.is_some());

        let result = set.find_with_options(
            "!@#$%",
            RegSetLead::Regex,
            SearchOptions::SEARCH_OPTION_NONE,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_regset_captures() {
        let set = RegSet::new(&[r"(\d+)-(\d+)", r"([a-z]+)"]).unwrap();

        if let Some((regex_index, captures)) = set.captures("hello123") {
            assert_eq!(regex_index, 1); // "[a-z]+" matches first by position
            assert_eq!(captures.at(0), Some("hello"));
            assert_eq!(captures.pos(0), Some((0, 5)));
        } else {
            panic!("Expected to find a match");
        }

        if let Some((regex_index, captures)) = set.captures("123-456") {
            assert_eq!(regex_index, 0); // First pattern with groups
            assert_eq!(captures.len(), 3); // Full match + 2 groups
            assert_eq!(captures.at(0), Some("123-456"));
            assert_eq!(captures.at(1), Some("123"));
            assert_eq!(captures.at(2), Some("456"));
        } else {
            panic!("Expected to find a match");
        }

        assert!(set.captures("!@#$%").is_none());
    }

    #[test]
    fn test_regset_replace_pattern() {
        let mut set = RegSet::new(&[r"\d+", r"[a-z]+"]).unwrap();

        assert!(set.find("123").is_some());
        set.replace_pattern(0, r"[A-Z]+").unwrap();
        assert!(set.replace_pattern(100, r"[A-Z]+").is_err());

        assert!(set.find("123").is_none());
        assert!(set.find("ABC").is_some());
        assert!(set.find("hello").is_some());
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_regset_add_pattern() {
        let mut set = RegSet::empty().unwrap();

        let idx1 = set.add_pattern(r"\d+").unwrap();
        assert_eq!(idx1, 0);
        assert_eq!(set.len(), 1);
        assert_eq!(set.find("hello123"), Some((0, 5)));

        let idx2 = set.add_pattern(r"[a-z]+").unwrap();
        assert_eq!(idx2, 1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.find("hello123"), Some((1, 0)));
    }

    #[test]
    fn test_regset_add_pattern_captures() {
        let mut set = RegSet::empty().unwrap();
        set.add_pattern(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

        let (idx, caps) = set.captures("2023-12-25").unwrap();
        assert_eq!(idx, 0);
        assert_eq!(caps.at(1), Some("2023"));
        assert_eq!(caps.at(2), Some("12"));
        assert_eq!(caps.at(3), Some("25"));
    }

    #[test]
    fn test_regset_add_pattern_errors() {
        let mut set = RegSet::empty().unwrap();

        assert!(set.add_pattern(r"[").is_err());
        assert_eq!(set.len(), 0);

        assert!(set.replace_pattern(0, r"\d+").is_err());

        set.add_pattern(r"\d+").unwrap();
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_regset_captures_with_encoding() {
        let set = RegSet::new(&[r"(\d+)", r"([a-z]+)"]).unwrap();

        if let Some((regex_index, captures)) = set.captures_with_encoding(
            "hello123",
            0,
            8,
            RegSetLead::Position,
            SearchOptions::SEARCH_OPTION_NONE,
        ) {
            assert_eq!(regex_index, 1); // "[a-z]+" matches first by position
            assert_eq!(captures.at(0), Some("hello"));
            assert_eq!(captures.at(1), Some("hello"));
        } else {
            panic!("Expected to find a match");
        }
    }
}
