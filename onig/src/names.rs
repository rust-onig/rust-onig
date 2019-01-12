use std::marker::PhantomData;
use std::iter::Iterator;
use std::ptr::null;
use std::str::from_utf8_unchecked;
use std::slice::from_raw_parts;
use std::os::raw::{c_int, c_uchar, c_uint, c_void};

use onig_sys::{self, OnigRegex, OnigUChar};

use super::Regex;

impl Regex {
    /// Returns the number of named groups into regex.
    pub fn capture_names_len(&self) -> usize {
        unsafe { onig_sys::onig_number_of_names(self.raw) as usize }
    }

    /// Returns the iterator over named groups as a tuple with the group name
    /// and group indexes.
    pub fn capture_names(&self) -> CaptureNames<'_> {
        CaptureNames {
            table: unsafe { (*self.raw).name_table as *const StTable },
            bin_idx: -1,
            entry_ptr: null(),
            _phantom: PhantomData,
        }
    }

    /// Calls `callback` for each named group in the regex. Each callback gets the group name
    /// and group indices.
    pub fn foreach_name<F>(&self, mut callback: F) -> i32
    where
        F: FnMut(&str, &[u32]) -> bool,
    {
        extern "C" fn foreach_cb<F>(
            name: *const OnigUChar,
            name_end: *const OnigUChar,
            ngroup_num: c_int,
            group_nums: *const c_int,
            _regex: OnigRegex,
            arg: *mut c_void,
        ) -> c_int
        where
            F: FnMut(&str, &[u32]) -> bool,
        {
            let name = unsafe {
                from_utf8_unchecked(from_raw_parts(name, name_end as usize - name as usize))
            };

            let groups = unsafe { from_raw_parts(group_nums as *const u32, ngroup_num as usize) };

            let callback = unsafe { &mut *(arg as *mut F) };

            if callback(name, groups) {
                0
            } else {
                -1
            }
        }

        unsafe {
            onig_sys::onig_foreach_name(
                self.raw,
                foreach_cb::<F>,
                &mut callback as *mut F as *mut c_void,
            )
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct NameEntry {
    name: *const c_uchar,
    name_len: c_int,
    back_num: c_int,
    back_alloc: c_int,
    back_ref1: c_int,
    back_refs: *const c_int,
}

// This is really `uintptr_t`. There isn't a deifnition for that in
// `os::raw`, but it is just defined as `usize` in `libc`. There's no
// point importing that whole crate just for the type definition.
#[cfg(windows)]
type StDataT = usize;

#[cfg(not(windows))]
type StDataT = ::std::os::raw::c_ulong;

#[repr(C)]
#[derive(Debug)]
struct StTableEntry {
    hash: c_uint,
    key: StDataT,
    record: StDataT,
    next: *const StTableEntry,
}

#[repr(C)]
#[derive(Debug)]
struct StTable {
    type_: *const c_void,
    num_bins: c_int,
    num_entries: c_int,
    bins: *const *const StTableEntry,
}

/// CaptureNames is an iterator over named groups as a tuple with the group name
/// and group indexes.
///
/// `'r` is the lifetime of the Regex object.
#[derive(Debug)]
pub struct CaptureNames<'r> {
    table: *const StTable,
    bin_idx: c_int,
    entry_ptr: *const StTableEntry,
    _phantom: PhantomData<&'r Regex>,
}

impl<'r> Iterator for CaptureNames<'r> {
    type Item = (&'r str, &'r [u32]);

    fn next(&mut self) -> Option<(&'r str, &'r [u32])> {
        unsafe {
            while self.entry_ptr.is_null() {
                if self.table.is_null() || self.bin_idx + 1 >= (*self.table).num_bins {
                    return None;
                }
                self.bin_idx += 1;
                self.entry_ptr = *(*self.table).bins.offset(self.bin_idx as isize)
            }
            let entry = (*self.entry_ptr).record as *const NameEntry;
            let name =
                from_utf8_unchecked(from_raw_parts((*entry).name, (*entry).name_len as usize));
            let groups = if (*entry).back_num > 1 {
                let ptr = (*entry).back_refs as *const u32;
                let len = (*entry).back_num as usize;
                from_raw_parts(ptr, len)
            } else {
                let ptr = &(*entry).back_ref1 as *const i32 as *const u32;
                from_raw_parts(ptr, 1)
            };
            self.entry_ptr = (*self.entry_ptr).next;
            Some((name, groups))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.table.is_null() {
            (0, None)
        } else {
            let size = unsafe { (*self.table).num_bins } as usize;
            (size, Some(size))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_regex_names_len() {
        let regex = Regex::new("(he)(l+)(o)").unwrap();
        assert_eq!(regex.capture_names_len(), 0);
        let regex = Regex::new("(?<foo>he)(?<bar>l+)(?<bar>o)").unwrap();
        assert_eq!(regex.capture_names_len(), 2);
        assert_eq!(regex.capture_histories_len(), 0);
    }

    #[test]
    fn test_regex_names() {
        let regex = Regex::new("(he)(l+)(o)").unwrap();
        let names = regex.capture_names().collect::<Vec<_>>();
        assert_eq!(names, vec![]);
        let regex = Regex::new("(?<foo>he)(?<bar>l+)(?<bar>o)").unwrap();
        let names = regex.capture_names().collect::<Vec<_>>();
        assert_eq!(
            names,
            [("foo", &[1u32] as &[u32]), ("bar", &[2u32, 3] as &[u32])]
        );
    }
}
