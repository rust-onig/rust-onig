//! Callout Support
//!
//! Callouts are registered on a regex and provide notifications as a
//! regex is matched.

/// The Callout Arguments Structure
///
/// This opaque type models access to the underlying callout arguments.
pub struct CalloutArgs(());

/// The Callout Trait
///
/// Callouts can be registered to receieve a notification when regex
/// matches are in progress.
pub trait Callout {

    /// On Callout
    ///
    /// Called when a regex match meets the criteria this callout was
    /// registered for.
    fn on_callout(&self, args: CalloutArgs);
}

impl<T> Callout for T where T: Fn(CalloutArgs) {
    fn on_callout(&self, args: CalloutArgs) {
        self(args)
    }
}
