# Change Log

This file contains the "big hitters" for each release. For more detailed
information about the exact changes in each release check the source code at
<https://github.com/rust-onig/rust-onig>.

## 7.0.0 (Unreleased)

 * `Regex` now carries a lifetime parameter `Regex<'_>` to bind it to any
   custom `Syntax` it was compiled with, preventing use-after-free.
 * MSRV bumped to 1.83.0 (affects both `onig` and `onig_sys`)
 * `onig_sys` bumped to 69.9.4 for the MSRV change
 * Expose RegSet in rust-onig
 * Expose 2 more SearchOption: SEARCH_OPTION_NOT_BEGIN_STRING and SEARCH_OPTION_NOT_BEGIN_POSITION

## 6.5.3

 * Fix build failure on BSD platforms due to missing `alloca.h`

## 6.5.2

 * Fixup Windows Build
 * Update bindgen and remove all warnings
 * Fix documentation link of `onig_sys` crate on crates.io
 * Fix heap-buffer-overflow in `Region::reserve()` via checked `usize` → `c_int` cast
 * Fix build failure on musl targets (e.g. Alpine Linux)

## 6.5.1

 * Version bumps

## 6.5.0

 * Upgrade `bitflags` to at least v2.4.0
 * MSRV bumped to 1.70.0

## 6.4.0

 * Upgrade to Rust 2018, #170
 * Replace `lazy_static` with `once_cell`

## 6.3.2

 * Bump Oniguruma
 * Set `cargo:rerun-if-env-changed` for `RUSTONIG` env variables
 * Ensure our environment variables start with `RUSTONIG`

## 6.3.0

 * MSRV bumped to 1.50.0
 * Proper support for round-tripping of syntax feature flags, including new
   flags that we don't recognise yet.
 * Support for the `p` syntax flag.

## 6.1.1

 * Use `repr(transparent)` on structs to ensure no UB
 * Fix for panics when encodings mis-match

## 6.1.0

 * Skip empty matches only when they overlap the last match (#145)
 * MSRV bumped to 1.40.0
 * Add support for WASM targets to the `onig_sys` build.

