//! Callout Support
//!
//! Callouts are registered on a regex and provide notifications as a
//! regex is matched.

use onig_sys::OnigCalloutIn;

/// The Callout Arguments Structure
///
/// This opaque type models access to the underlying callout arguments.
#[derive(Debug)]
pub struct CalloutArgs {
    raw: *mut onig_sys::OnigCalloutArgs,
}

/// The Callout Arguments
///
/// This struct wraps the information passed in to each callback.
impl CalloutArgs {
    pub(crate) fn from_raw(raw: *mut onig_sys::OnigCalloutArgs) -> Self {
        CalloutArgs { raw }
    }

    /// Returns the callout number of this callout. "callout number" is an
    /// idneitifier of callout in a regex pattern.
    pub fn callout_num(&self) -> i32 {
        unsafe { onig_sys::onig_get_callout_num_by_callout_args(self.raw) }
    }

    /// Returns the direction of this callout.
    pub fn callout_in(&self) -> CalloutIn {
        CalloutIn::from(unsafe { onig_sys::onig_get_callout_in_by_callout_args(self.raw) })
    }

    /// Returns the name identifier of this callout. Fi this callout is callout
    /// of contents, then returns None.
    pub fn name_id(&self) -> Option<i32> {
        let name = unsafe { onig_sys::onig_get_name_id_by_callout_args(self.raw) };
        if name == onig_sys::ONIG_NON_NAME_ID {
            None
        } else {
            Some(name)
        }
    }

    /// Returns the current counter value for retry-limit-in-match.
    pub fn retry_counter(&self) -> u64 {
        unsafe { onig_sys::onig_get_retry_counter_by_callout_args(self.raw) }
    }

    /// Returns current used match-stack size.
    ///
    /// The returned tuple `(used_num, used_bytes)` is made up of:
    ///
    ///   * `used_num` - number of match-srtack elements
    ///   * `used_bytes` - used byte size of match stack
    pub fn used_stack_size(&self) -> Option<(i32, i32)> {
        let mut used_num = 0;
        let mut used_bytes = 0;
        let r = unsafe {
            onig_sys::onig_get_used_stack_size_in_callout(
                self.raw,
                (&mut used_num) as *mut _,
                (&mut used_bytes) as *mut _,
            )
        };

        if r == 0 {
            Some((used_num, used_bytes))
        } else {
            None
        }
    }
}

/// Callout Type
#[derive(Debug)]
pub enum CalloutIn {
    /// The current callout is a progress callout.
    Progress,
    /// The current callout is a retraction callout.
    Retraction,
}

impl From<OnigCalloutIn> for CalloutIn {
    fn from(i: OnigCalloutIn) -> Self {
        match i {
            onig_sys::OnigCalloutIn_ONIG_CALLOUT_IN_PROGRESS => CalloutIn::Progress,
            onig_sys::OnigCalloutIn_ONIG_CALLOUT_IN_RETRACTION => CalloutIn::Retraction,
            _ => panic!("Invalid value for CalloutIn"),
        }
    }
}

/// Callout Result
pub enum CalloutResult {
    /// The callout succeeded. Matching should continue.
    Success,
    /// The callout failed. Matching should fail.
    Fail,
    /// The callout encountered an error.
    Error(u32),
}

impl From<CalloutResult> for ::std::os::raw::c_int {
    fn from(result: CalloutResult) -> Self {
        match result {
            CalloutResult::Success => 0,
            CalloutResult::Fail => 1,
            CalloutResult::Error(code) => -(code as i32),
        }
    }
}

/// The Callout Trait
///
/// Callouts can be registered to receieve a notification when regex
/// matches are in progress.
pub trait Callout {
    /// On Match Callback
    ///
    /// Called when a regex match meets the criteria this callout was
    /// registered for.
    fn on_match_progress(&self, args: CalloutArgs) -> CalloutResult;

    /// On Retraction Callback
    ///
    /// Called when a regex match retraction meets the criteria this callout was
    /// registered for.
    fn on_retraction(&mut self, args: CalloutArgs) -> CalloutResult;
}

impl<T> Callout for T
where
    T: Fn(CalloutArgs) -> CalloutResult,
{
    fn on_match_progress(&self, args: CalloutArgs) -> CalloutResult {
        self(args)
    }

    fn on_retraction(&mut self, args: CalloutArgs) -> CalloutResult {
        self(args)
    }
}
