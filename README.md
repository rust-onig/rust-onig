# Rust Onig

Rust bindings for the [Oniguruma regex library][Onig_wiki].

**This crate is still under development at the moment.**

#### Example of usage

```rust
use onig::Regex;

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


## Rust Onig is Open Source

The contents of this repository are distributed under the MIT license. See [LICENSE](LICENSE.md) for more details.

 [Onig_wiki]: https://en.wikipedia.org/wiki/Oniguruma
