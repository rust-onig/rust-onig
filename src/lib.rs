//! This crate provides a safe wrapper around the
//! [Oniguruma](https://github.com/kkos/oniguruma) regular expression library.
//!
//! # Examples
//!
//! ```rust
//! use onig::Regex;
//!
//! let regex = Regex::new("e(l+)").unwrap();
//! for (i, pos) in regex.captures("hello").unwrap().iter_pos().enumerate() {
//!     match pos {
//!          Some((beg, end)) =>
//!              println!("Group {} captured in position {}:{}", i, beg, end),
//!          None =>
//!              println!("Group {} is not captured", i)
//!     }
//! }
//! ```

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate onig_sys;

mod captures;
mod flags;
mod region;
mod regex;
mod syntax;
mod tree;
mod utils;

// re-export the onig types publically
pub use flags::*;
pub use region::Region;
pub use regex::{Regex, Error};
pub use captures::{Captures, SubCaptures, SubCapturesPos};
pub use tree::{CaptureTreeNode, CaptureTreeNodeIter};
pub use syntax::Syntax;
pub use utils::version;
