# Rust Onig

[![Build Status](https://travis-ci.org/rust-onig/rust-onig.svg?branch=master)](https://travis-ci.org/rust-onig/rust-onig)

Rust bindings for the [Oniguruma regex library][Onig_wiki], a powerful and mature regular expression library with support for a wide range of character sets and language syntaxes. Oniguruma is written in C. This repository provides two crates: `onig-sys` which provides the raw Rust FFI bindings, and `onig`, which provides a safe Rust wrapper around them.

## Documentation

Check out the [module documentation][onig_crate_doc] to find out all the features that are available. To see some example usage of this crate take a look a the [examples folder][examples_folder]. The examples can be run from the command line with `cargo run --example <examplename>`.

## Getting Started

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
	onig = "0.3"
```

Add the following extern to your crate root:

```rust
extern crate onig;
```

You can can compile simple regular expressions with [`Regex::new`][regex_new], check if the pattern matches an entire `&str`  with [`Regex::is_match`][regex_is_match] and find matches within a `&str` with [`Regex::find`][regex_find]. The `onig` crate also supplies more powerful versions of these methods which expose the wide range of options Oniguruma provides.

```rust
use onig::*;

let regex = Regex::new("e(l+)").unwrap();
for (i, pos) in regex.captures("hello").unwrap().iter_pos().enumerate() {
    match pos {
         Some((beg, end)) =>
             println!("Group {} captured in position {}:{}", i, beg, end),
         None =>
             println!("Group {} is not captured", i)
    }
}
```


## Rust-Onig is Open Source

The contents of this repository are distributed under the MIT license. See [LICENSE](LICENSE.md) for more details.

 [Onig_wiki]: https://en.wikipedia.org/wiki/Oniguruma
 [onig_crate_doc]: http://rust-onig.github.io/rust-onig/onig/
 [examples_folder]: https://github.com/rust-onig/rust-onig/tree/master/examples
 [regex_new]: http://rust-onig.github.io/rust-onig/onig/struct.Regex.html#method.new
 [regex_is_match]: http://rust-onig.github.io/rust-onig/onig/struct.Regex.html#method.is_match
 [regex_find]: http://rust-onig.github.io/rust-onig/onig/struct.Regex.html#method.find