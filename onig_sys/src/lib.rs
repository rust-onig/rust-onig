mod legacy;
pub mod bindgened;

pub use self::legacy::*;

#[test]
fn test_is_linked() {
    unsafe {
        assert!(!onig_copyright().is_null());
        assert!(!onig_version().is_null());
    }
}
