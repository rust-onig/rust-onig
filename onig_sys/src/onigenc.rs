//! Onig Encoding API
//!
//! This api contains functions for working with oniguruma encodings.

use super::{OnigEncoding, OnigUChar};

use libc::c_int;

extern "C" {

    /// Onigenc Init
    ///
    /// ```c
    /// int onigenc_init(void);
    /// ```
    pub fn onigenc_init() -> c_int;

    /// Onigenc Step Back
    ///
    /// ```c
    /// OnigUChar* onigenc_step_back (OnigEncoding enc,
    ///                               const OnigUChar* start,
    ///                               const OnigUChar* s,
    ///                               int n);
    /// ```
    pub fn onigenc_step_back(enc: OnigEncoding,
                             start: *const OnigUChar,
                             s: *const OnigUChar,
                             n: c_int)
                             -> *const OnigUChar;
    
    /// Onigenc Set Default Encoding
    ///
    /// ```c
    /// int onigenc_set_default_encoding(OnigEncoding enc);
    /// ```
    pub fn onigenc_set_default_encoding(enc: OnigEncoding);

    /// Onigenc Get Default Encoding
    ///
    /// ```c
    /// OnigEncoding onigenc_get_default_encoding(void);
    /// ```
    pub fn onigenc_get_default_encoding() -> OnigEncoding;
    
    /// Onigenc Set Default Case Conversion Table
    ///
    /// ```c
    /// void  onigenc_set_default_caseconv_table(const OnigUChar* table);
    /// ```
    pub fn onigenc_set_default_caseconv_table(table: *const OnigUChar);
    
    /// Onigenc Get Right Adjust Char Head With Prev
    ///
    /// ```c
    /// OnigUChar* onigenc_get_right_adjust_char_head_with_prev(
    ///     OnigEncoding enc,
    ///     const OnigUChar* start,
    ///     const OnigUChar* s,
    ///     const OnigUChar** prev);
    pub fn onigenc_get_right_adjust_char_head_with_prev(
        enc: OnigEncoding,
        start: *const OnigUChar,
        s: *const OnigUChar,
        prev: *mut *const OnigUChar)
        -> *const OnigUChar;

    ///   Return previous character head address.
    ///
    ///  `UChar* onigenc_get_prev_char_head(OnigEncoding enc, const UChar* start, const UChar* s)`
    ///
    ///   arguments
    ///   1 enc:   character encoding
    ///   2 start: string address
    ///   3 s:     target address of string
    pub fn onigenc_get_prev_char_head(enc: OnigEncoding,
                                      start: *const OnigUChar,
                                      s: *const OnigUChar)
                                      -> *const OnigUChar;

    ///   Return left-adjusted head address of a character.
    ///
    ///  `UChar* onigenc_get_left_adjust_char_head(OnigEncoding enc,
    ///                                            const UChar* start, const UChar* s)`
    ///
    /// # Arguments
    ///
    ///   1. enc:   character encoding
    ///   2. start: string address
    ///   3. s:     target address of string
    pub fn onigenc_get_left_adjust_char_head(enc: OnigEncoding,
                                             start: *const OnigUChar,
                                             s: *const OnigUChar)
                                             -> *const OnigUChar;

    ///   Return right-adjusted head address of a character.
    ///
    ///  `UChar* onigenc_get_right_adjust_char_head(OnigEncoding enc,
    ///                                             const UChar* start, const UChar* s)`
    ///
    /// # Arguments
    ///
    ///   1. enc:   character encoding
    ///   2. start: string address
    ///   3. s:     target address of string
    pub fn onigenc_get_right_adjust_char_head(enc: OnigEncoding,
                                              start: *const OnigUChar,
                                              s: *const OnigUChar)
                                              -> *const OnigUChar;

    ///   Return number of characters in the string.
    ///
    ///  `int onigenc_strlen(OnigEncoding enc, const UChar* s, const UChar* end)`
    pub fn onigenc_strlen(enc: OnigEncoding, s: *const OnigUChar, end: *const OnigUChar) -> c_int;

    ///   Return number of characters in the string.
    ///
    ///  `int onigenc_strlen_null(OnigEncoding enc, const UChar* s)`
    pub fn onigenc_strlen_null(enc: OnigEncoding, s: *const OnigUChar) -> c_int;

    ///   Return number of bytes in the string.
    ///
    ///  `int onigenc_str_bytelen_null(OnigEncoding enc, const UChar* s)`
    pub fn onigenc_str_bytelen_null(enc: OnigEncoding, s: *const OnigUChar) -> c_int;

}
