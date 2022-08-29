//! Match Parameters
//!
//! Contains the definition for the `MatchParam` struct. This can be
//! used to control the behavior of searching and matching.

use std::os::raw::{c_uint, c_ulong};

use crate::callout::{Callout, CalloutArgs};

/// Parameters for a Match or Search.
pub struct MatchParam {
    raw: *mut onig_sys::OnigMatchParam,
}

impl MatchParam {
    /// Set the match stack limit
    pub fn set_match_stack_limit(&mut self, limit: u32) {
        unsafe {
            onig_sys::onig_set_match_stack_limit_size_of_match_param(self.raw, limit as c_uint);
        }
    }

    /// Set the retry limit in match
    pub fn set_retry_limit_in_match(&mut self, limit: u32) {
        unsafe {
            onig_sys::onig_set_retry_limit_in_match_of_match_param(self.raw, c_ulong::from(limit));
        }
    }

    /// Set the retry limit in search
    pub fn set_retry_limit_in_search(&mut self, limit: u32) {
        unsafe {
            onig_sys::onig_set_retry_limit_in_search_of_match_param(self.raw, c_ulong::from(limit));
        }
    }

    /// Get the Raw `OnigMatchParam` Pointer
    pub(crate) fn as_raw(&self) -> *mut onig_sys::OnigMatchParam {
        self.raw
    }

    /// Add callout data to the match param.
    pub fn add_callout<C: Callout + 'static>(&mut self, callout: C) {
        let callout = Box::into_raw(Box::new(callout));
        unsafe {
            onig_sys::onig_set_callout_user_data_of_match_param(self.raw, callout as *mut _);
            onig_sys::onig_set_progress_callout_of_match_param(
                self.raw,
                Some(callout_progress_thunk::<C>),
            );
            onig_sys::onig_set_retraction_callout_of_match_param(
                self.raw,
                Some(callout_retraction_thunk::<C>),
            );
        }

        unsafe extern "C" fn callout_progress_thunk<C: Callout>(
            args: *mut onig_sys::OnigCalloutArgs,
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int {
            let callout = user_data as *mut C;
            let callout = &mut *callout;

            let args = CalloutArgs::from_raw(args);

            callout.on_match_progress(args).into()
        }

        unsafe extern "C" fn callout_retraction_thunk<C: Callout>(
            args: *mut onig_sys::OnigCalloutArgs,
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int {
            let callout = user_data as *mut C;
            let callout = &mut *callout;

            let args = CalloutArgs::from_raw(args);

            callout.on_retraction(args).into()
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

    #[test]
    pub fn set_retry_limit_in_match() {
        let mut mp = MatchParam::default();
        mp.set_retry_limit_in_match(1000);
    }
}
