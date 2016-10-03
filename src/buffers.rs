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

impl <T> EncodedStringBuffer for T where T: AsRef<str> {

    fn start_ptr(&self) -> *const onig_sys::OnigUChar {
        self.as_ref().as_bytes().as_ptr()
    }

    fn limit_ptr(&self) -> *const onig_sys::OnigUChar {
        let bytes = self.as_ref().as_bytes();
        bytes[bytes.len()..].as_ptr()
    }
}

/// Byte Buffer
///
/// Represents a buffer of bytes, with an encoding.
pub struct ByteBuffer<'a> {
    bytes: &'a[u8],
    enc: onig_sys::OnigEncoding
}

impl<'a> ByteBuffer<'a> {

    /// New Buffer from Parts
    ///
    /// # Arguments
    ///
    ///  * `bytes` - The contents of the buffer
    ///  * `enc` - The encoding this buffer is in
    ///
    /// # Returns
    ///
    /// A new buffer instance
    pub fn from_parts(bytes: &'a[u8], enc: onig_sys::OnigEncoding) -> ByteBuffer<'a> {
        ByteBuffer {
            bytes: bytes,
            enc: enc
        }
    }

    /// New ASCII Buffer
    ///
    /// # Arguments
    ///
    ///  * `bytes` - The ASCII encoded string
    ///
    /// # Returns
    ///
    /// A new buffer instance
    pub fn ascii(bytes: &'a[u8]) -> ByteBuffer<'a> {
        ByteBuffer {
            bytes: bytes,
            enc: &onig_sys::OnigEncodingASCII
        }
    }
}

impl<'a> EncodedStringBuffer for ByteBuffer<'a> {

    fn start_ptr(&self) -> *const onig_sys::OnigUChar {
        self.bytes.as_ptr()
    }

    fn limit_ptr(&self) -> *const onig_sys::OnigUChar {
        self.bytes[self.bytes.len()..].as_ptr()
    }

    fn encoding(&self) -> onig_sys::OnigEncoding {
        self.enc
    }
}

#[cfg(test)]
pub mod tests {

    use onig_sys;

    use super::*;

    #[test]
    pub fn rust_string_encoding_is_utf8() {
        let foo = "foo";
        assert_eq!(&onig_sys::OnigEncodingUTF8 as onig_sys::OnigEncoding, foo.encoding());

        let bar = String::from(".*");
        assert_eq!(&onig_sys::OnigEncodingUTF8 as onig_sys::OnigEncoding, bar.encoding());
    }

    #[test]
    pub fn rust_bytes_encoding_is_ascii() {
        let fizz = b"fizz";
        let buff = ByteBuffer::ascii(fizz);
        assert_eq!(&onig_sys::OnigEncodingASCII as onig_sys::OnigEncoding, buff.encoding());
    }

    #[test]
    pub fn rust_string_ptr_offsets_are_valid() {
        let test_string = "hello world";
        assert_eq!(test_string.limit_ptr() as usize - test_string.start_ptr() as usize, test_string.len());
    }

    #[test]
    pub fn rust_bytes_ptr_offsets_are_valid() {
        let fozz = b"foo.*bar";
        let buff = ByteBuffer::ascii(fozz);
        assert_eq!(buff.limit_ptr() as usize - buff.start_ptr() as usize, fozz.len());
    }

    #[test]
    pub fn byte_buffer_create() {
        let buff = b"hello world";
        let enc_buffer = ByteBuffer::from_parts(buff, &onig_sys::OnigEncodingASCII);
        assert_eq!(&onig_sys::OnigEncodingASCII as onig_sys::OnigEncoding, enc_buffer.encoding());
        assert_eq!(enc_buffer.limit_ptr() as usize - enc_buffer.start_ptr() as usize, buff.len());
    }
}
