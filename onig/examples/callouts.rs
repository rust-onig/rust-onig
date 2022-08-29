use onig::{MatchParam, Regex, SearchOptions, callout::CalloutArgs};

fn test(match_param: &MatchParam, pattern: &str, haystack: &str) {
    let reg = Regex::new(pattern).unwrap();

    println!(
        "result: {:?}",
        reg.search_with_param(
            haystack,
            0,
            haystack.len(),
            SearchOptions::SEARCH_OPTION_NONE,
            None,
            match_param
        )
    );
}

fn main() {
    let mut mp = MatchParam::default();

    mp.add_callout(|args: CalloutArgs| {
        println!("Callout: {:?}", args.callout_in());
    });

    test(&mp, "a+(?{foo bar baz...}X)$", "aaab");
    test(
        &mp,
        "(?{{!{}#$%&'()=-~^|[_]`@*:+;<>?/.\\,}}[symbols])c",
        "abc",
    );
    test(
        &mp,
        "\\A(...)(?{{{booooooooooooo{{ooo}}ooooooooooz}}}<)",
        "aaab",
    );
    test(&mp, "\\A(?!a(?{in prec-read-not}[xxx]X)b)", "ac");
    test(&mp, "(?<!a(?{in look-behind-not}X)c)c", "abc");
}
