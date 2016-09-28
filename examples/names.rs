extern crate onig;

use onig::*;

fn main() {
    let pattern = "(?<foo>a*)(?<bar>b*)(?<foo>c*)";
    let string = "aaabbbbcc";

    let r = Regex::new(pattern).unwrap();

    println!("has {} group names:", r.capture_names_len());

    for (name, indices) in r.capture_names() {
        println!("- {}: {:?}", name, indices);
    }

    if let Some(position) = r.search_with_options(string, 0, string.len(),
                                                  SEARCH_OPTION_NONE, None)
    {
        println!("match at {}", position);
    } else {
        println!("search fail")
    }
}
