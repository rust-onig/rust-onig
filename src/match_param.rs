//! Match Parameters
//!
//! Contains the definition for the `MatchParam` struct. This can be
//! used to control the behavior of searching and matching.

use onig_sys;
use libc::c_uint;

/// Parameters for a Match or Search.
pub struct MatchParam {
    raw: *mut onig_sys::OnigMatchParam,
}

impl MatchParam {
    /// Set the match statck limit
    pub fn set_match_stack_limit(&mut self, limit: u32) {
        unsafe {
            onig_sys::onig_set_match_stack_limit_size_of_match_param(
                self.raw,
                limit as c_uint
            );
        }
    }
}

impl Default for MatchParam {
    fn default() -> Self {
        let raw = unsafe {
            let new = onig_sys::onig_new_match_param();
            onig_sys::onig_initialize_match_param(new);
            new
        };
        MatchParam { raw }
    }
}

impl Drop for MatchParam {
    fn drop(&mut self) {
        unsafe {
            onig_sys::onig_free_match_param(self.raw);
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn create_default_match_param() {
        let _mp = MatchParam::default();
    }

    #[test]
    pub fn set_max_stack_size_limit() {
        let mut mp = MatchParam::default();
        mp.set_match_stack_limit(1000);
    }
}
