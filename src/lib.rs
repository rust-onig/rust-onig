extern crate libc;

use libc::c_int;

mod onig_sys;
mod utils;

#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct OnigRegion {
    raw: *const onig_sys::OnigRegion
}

impl OnigRegion {
    pub fn new() -> OnigRegion {
        let raw = unsafe {
            onig_sys::onig_region_new()
        };
        OnigRegion { raw:raw }
    }

    pub fn clear(&mut self) {
        unsafe {
            onig_sys::onig_region_clear(self.raw);
        }
    }
    
    pub fn resize(&mut self, new_size: usize) -> usize {
        unsafe {
            onig_sys::onig_region_resize(self.raw, new_size as c_int) as usize
        }
    }
}

impl Drop for OnigRegion {
    fn drop(&mut self) {
        unsafe {
            onig_sys::onig_region_free(self.raw, 1);
        }
    }
}



#[cfg(test)]
mod test_lib {

    use super::*;

    #[test]
    fn test_region_create() {
        OnigRegion::new();
    }

    #[test]
    fn test_region_clear() {
        let mut region = OnigRegion::new();
        region.clear();
    }
}
