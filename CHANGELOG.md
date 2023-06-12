# Change Log

This file contains the "big hitters" for each release. For more detailed
information about the exact changes in each release check the source code at
<https://github.com/rust-onig/rust-onig>.

## 6.5.0

 * Bump Bindgen for Clang 16 support.
 * Bump MSRV to 2021 (1.60.0)

## 6.4.0

 * Upgrade to Rust 2018, #170
 * Replace `lazy_static` with `once_cell`

## 6.3.2

 * Bump Oniguruma
 * Set `cargo:rerun-if-env-changed` for `RUSTONIG` env variables
 * Ensure our environment variables start with `RUSTONIG`

## 6.3.0

 * MSRV bumped to 1.50.0
 * Proper support for round-tripping of syntax feature falgs, including new
   flags that we don't recognise yet.
 * Support for the `p` syntax flag.

## 6.1.1

 * Use `repr(transparent)` on structs to ensure no UB
 * Fix for panics when encodings mis-match

## 6.1.0

 * Skip empty matches only when they overlap the last match (#145)
 * MSRV bumped to 1.40.0
 * Add support for WASM targets to the `onig_sys` build.

