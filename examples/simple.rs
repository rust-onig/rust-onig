extern crate onig;

use onig::*;

fn main() {
    let pattern = "a(.*)b|[e-f]+";
    let string = "zzzzaffffffffb";

    let r = Regex::with_options(pattern, REGEX_OPTION_NONE, Syntax::default())
        .unwrap();

    let mut region = Region::new();

    match r.search_with_options(
        string, 0, string.len(), SEARCH_OPTION_NONE, Some(&mut region)) {
        Some(_) => {
            let mut i = 0;
            while let Some(pos) = region.pos(i) {
                println!("{}: {:?}", i, pos);
                i += 1;
            }
        },
        None => println!("search fail")
    }
}
