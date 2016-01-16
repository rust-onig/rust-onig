use libc::*;
use onig_sys;

/// Onig Region
///
/// Represents a set of capture groups found in a search or match.
#[derive(Debug)]
pub struct OnigRegion {
    raw: *mut onig_sys::OnigRegion,
}

impl OnigRegion {
    /// Create a new empty `OnigRegion`
    ///
    /// Creates an onig region object which can be used to collect
    /// matches. See [`onig_sys::onig_region_new`][region_new] for
    /// more info.
    ///
    /// [region_new]: ./onig_sys/fn.onig_region_new.html
    pub fn new() -> OnigRegion {
        let raw = unsafe {
            onig_sys::onig_region_new()
        };
        OnigRegion { raw: raw }
    }

    /// Create a new region with a given size. This function allocates
    /// a new region object as in `OnigRegion::new` and resizes it to
    /// contain at least `capacity` regions.
    ///
    /// # Arguments
    ///
    /// * `capacity` - the number of captures this region should be
    /// capable of storing without allocation.
    pub fn with_capacity(capacity: usize) -> OnigRegion {
        let mut region = Self::new();
        region.reserve(capacity);
        region
    }

    /// Clear the Region
    ///
    /// This can be used to clear out a region so it can be used
    /// again. See [`onig_sys::onig_region_clear`][region_clear]
    ///
    /// [region_clear]: ./onig_sys/fn.onig_region_clear.html
    ///
    /// # Arguments
    ///
    ///  * `self` - The region to clear
    pub fn clear(&mut self) {
        unsafe {
            onig_sys::onig_region_clear(self.raw);
        }
    }

    pub fn capacity(&self) -> usize {
        unsafe {
            (*self.raw).allocated as usize
        }
    }

    /// Resize the Region
    ///
    /// Updates the region to contain `new_capacity` slots. See
    /// [`onig_sys::onig_region_resize`][region_resize] for mor
    /// information.
    ///
    /// [region_resize]: ./onig_sys/fn.onig_region_resize.html
    ///
    /// # Arguments
    ///
    ///  * `self` - The region to resize
    ///  * `new_capacity` - The new number of groups in the region.
    pub fn reserve(&mut self, new_capacity: usize) {
        let r = unsafe {
            onig_sys::onig_region_resize(self.raw, new_capacity as c_int)
        };
        if r != 0 {
            panic!("Onig: Memory overflow during region resize")
        }
    }

    /// Get the size of the region. Returns the number of registers in
    /// the region.
    pub fn len(&self) -> usize {
        unsafe {
            (*self.raw).num_regs as usize
        }
    }
}

/// Clears up the underlying Oniguruma object. When dropped calls
/// [`onig_sys::onig_region_free`][region_free] on the contained raw
/// onig region pointer.
///
/// [region_free]: ./onig_sys/fn.onig_region_free.html
impl Drop for OnigRegion {
    fn drop(&mut self) {
        unsafe {
            onig_sys::onig_region_free(self.raw, 1);
        }
    }
}

#[cfg(test)]
mod tests {

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

    #[test]
    fn test_region_resize() {
        {
            let mut region = OnigRegion::new();
            assert!(region.len() == 0);
            region.reserve(100);
            {
                // can still get the size without a mutable borrow
                let region_borrowed = &region;
                assert!(region_borrowed.capacity() == 100);
            }
        }

        {
            let region = OnigRegion::with_capacity(10);
            assert!(region.capacity() == 10);
        }
    }
}
