use std::ptr::null;
use std::mem::transmute;
use libc::c_int;
use onig_sys;

use super::CaptureTreeNode;

/// Represents a set of capture groups found in a search or match.
#[derive(Debug, Eq, PartialEq)]
pub struct Region {
    raw: onig_sys::OnigRegion,
}

impl Region {
    /// Create a new empty `Region`
    pub fn new() -> Region {
        Region {
            raw: onig_sys::OnigRegion {
                allocated: 0,
                num_regs: 0,
                beg: null(),
                end: null(),
                history_root: null(),
            },
        }
    }

    /// Create a new region with a given capacity. This function allocates
    /// a new region object as in `Region::new` and resizes it to
    /// contain at least `capacity` regions.
    ///
    /// # Arguments
    ///
    /// * `capacity` - the number of captures this region should be
    /// capable of storing without allocation.
    pub fn with_capacity(capacity: usize) -> Region {
        let mut region = Self::new();
        region.reserve(capacity);
        region
    }

    /// This can be used to clear out a region so it can be used
    /// again. See [`onig_sys::onig_region_clear`][region_clear]
    ///
    /// [region_clear]: ./onig_sys/fn.onig_region_clear.html
    pub fn clear(&mut self) {
        unsafe {
            onig_sys::onig_region_clear(&mut self.raw);
        }
    }

    pub fn capacity(&self) -> usize {
        self.raw.allocated as usize
    }

    /// Updates the region to contain `new_capacity` slots. See
    /// [`onig_sys::onig_region_resize`][region_resize] for mor
    /// information.
    ///
    /// [region_resize]: ./onig_sys/fn.onig_region_resize.html
    ///
    /// # Arguments
    ///
    ///  * `new_capacity` - The new number of groups in the region.
    pub fn reserve(&mut self, new_capacity: usize) {
        let r = unsafe { onig_sys::onig_region_resize(&mut self.raw, new_capacity as c_int) };
        if r != onig_sys::ONIG_NORMAL {
            panic!("Onig: fail to memory allocation during region resize")
        }
    }

    /// Get the size of the region. Returns the number of registers in
    /// the region.
    pub fn len(&self) -> usize {
        self.raw.num_regs as usize
    }

    /// Returns the start and end positions of the Nth capture group. Returns
    /// `None` if `pos` is not a valid capture group or if the capture group did
    /// not match anything. The positions returned are always byte indices with
    /// respect to the original string matched.
    pub fn pos(&self, pos: usize) -> Option<(usize, usize)> {
        if pos >= self.len() {
            return None;
        }
        let (beg, end) = unsafe {
            (*self.raw.beg.offset(pos as isize),
             *self.raw.end.offset(pos as isize))
        };
        if beg != onig_sys::ONIG_REGION_NOTPOS {
            Some((beg as usize, end as usize))
        } else {
            None
        }
    }

    pub fn tree(&self) -> Option<&CaptureTreeNode> {
        let tree = unsafe { onig_sys::onig_get_capture_tree(&self.raw) };
        if tree.is_null() {
            None
        } else {
            Some(unsafe { transmute(tree) })
        }
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe {
            onig_sys::onig_region_free(&mut self.raw, 0);
        }
    }
}

impl Clone for Region {
    fn clone(&self) -> Self {
        let mut new_region = Region::new();
        unsafe {
            onig_sys::onig_region_copy(&mut new_region.raw, &self.raw);
        }
        new_region
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_create() {
        Region::new();
    }

    #[test]
    fn test_region_clear() {
        let mut region = Region::new();
        region.clear();
    }

    #[test]
    fn test_region_copy() {
        let region = Region::new();
        let new_region = region.clone();
        assert_eq!(new_region.len(), region.len());
    }

    #[test]
    fn test_region_resize() {
        {
            let mut region = Region::new();
            assert!(region.capacity() == 0);
            region.reserve(100);
            {
                // can still get the capacity without a mutable borrow
                let region_borrowed = &region;
                assert!(region_borrowed.capacity() == 100);
            }
        }

        {
            let region = Region::with_capacity(10);
            assert!(region.capacity() == 10);
        }
    }
}
