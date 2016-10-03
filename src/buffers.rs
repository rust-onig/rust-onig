//! Encoded Buffers Implementation
//!
//! This module contains a trait used for converting byte buffers or
//! Rust strings into oniguruma char buffers to search and compile
//! with.

use onig_sys;

/// Encoded String Buffer
///
/// Represents a buffer of characters with encoding information
/// attached.
pub trait EncodedStringBuffer {

    /// Pointer to the start of the pattern
    ///
    /// This should point to the first character in the buffer,
    /// encoded as an `onig_sys` character.
    fn start_ptr(&self) -> *const onig_sys::OnigUChar;

    /// Pointer to the limit of the pattern buffer
    ///
    /// This should point just past the final character in the buffer,
    /// encoded as an `onig_sys` character.
    fn limit_ptr(&self) -> *const onig_sys::OnigUChar;

    /// The encoding of the contents of the buffer
    fn encoding(&self) -> onig_sys::OnigEncoding {
        &onig_sys::OnigEncodingUTF8
    }
}

impl <T> EncodedStringBuffer for T where T : AsRef<str> {

    fn start_ptr(&self) -> *const onig_sys::OnigUChar {
        self.as_ref().as_bytes().as_ptr()
    }

    fn limit_ptr(&self) -> *const onig_sys::OnigUChar {
        let bytes = self.as_ref().as_bytes();
        bytes[bytes.len()..].as_ptr()
    }
}
