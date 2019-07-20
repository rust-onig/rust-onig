pub mod bindgened;

pub use self::bindgened::*;

// backfill types from the old hand-written bindings:

pub type OnigSyntaxBehavior = ::std::os::raw::c_uint;
pub type OnigSyntaxOp = ::std::os::raw::c_uint;
pub type OnigSyntaxOp2 = ::std::os::raw::c_uint;

#[test]
fn test_is_linked() {
    unsafe {
        assert!(!onig_copyright().is_null());
        assert!(!onig_version().is_null());
    }
}
