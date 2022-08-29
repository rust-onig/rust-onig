use onig::{
    callout::{CalloutArgs, CalloutResult},
    MatchParam, Regex, SearchOptions,
};

fn test(match_param: &MatchParam, pattern: &str, haystack: &str) {
    let reg = match Regex::new(pattern) {
        Ok(reg) => reg,
        Err(e) => {
            eprintln!("Error creating pattern: {}", e);
            return;
        }
    };

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

    mp.set_callout(|args: CalloutArgs| {
        println!(
            "Callout: {:?} ({:?}, {:?}, {:?}) {:?}",
            args.callout_num(),
            args.callout_in(),
            args.name_id().and_then(onig::callout::get_callout_name),
            args.retry_counter(),
            args.used_stack_size()
        );
        CalloutResult::Success
    });

  /* callout of contents */
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

    // callout of name
    test(&mp, "\\A(*foo)abc", "abc");
    test(&mp, "abc(?:(*FAIL)|$)", "abcabc");
    test(&mp, "abc(?:$|(*MISMATCH)|abc$)", "abcabc");
    test(&mp, "abc(?:(*ERROR)|$)", "abcabc");
    test(&mp, "ab(*foo{})(*FAIL)", "abc");
    test(&mp, "abc(d|(*ERROR{-999}))", "abc");
    test(&mp, "ab(*bar{372,I am a bar's argument,あ})c(*FAIL)", "abc");
    test(&mp, "ab(*bar{1234567890})", "abc");
    test(&mp, "(?:a(*MAX{2})|b)*", "abbabbabbabb");
    test(&mp, "(?:(*MAX{2})a|b)*", "abbabbabbabb");
    test(&mp, "(?:(*MAX{1})a|b)*", "bbbbbabbbbbabbbbb");
    test(&mp, "(?:(*MAX{3})a|(*MAX{4})b)*", "bbbaabbab");
    test(
        &mp,
        "(?:(*MAX[A]{3})a|(*MAX[B]{5})b)*(*CMP{A,<,B})",
        "abababc",
    );
    test(
        &mp,
        "(?:(*MAX[A]{7})a|(*MAX[B]{5})b)*(*CMP{A,>=,4})",
        "abababcabababaa",
    );
    test(&mp, "(?:(*MAX[T]{3})a)*(?:(*MAX{T})c)*", "aaccc");

    /* callouts in condition */
    test(&mp, "\\A(?(?{in condition})then|else)\\z", "then");
    test(&mp, "\\A(?(*FAIL)then|else)\\z", "else");

    /* monitor test */
    test(&mp, "(?:(*MON{X})(*FAIL)|.{,3}(*MON[FOO])k)", "abcdefghijk");
}
