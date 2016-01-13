extern crate onig;

use onig::*;
use std::env;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut regexes = HashMap::new();
    for arg in env::args().skip(1) {
        println!("Compiling '{}'", arg);
        let regex_compilation = Regex::new_with_options(
            &arg,
            onig::options::ONIG_OPTION_SINGLELINE,
            onig::syntax_types::EMACS);
        match regex_compilation {
            Ok(regex) => {regexes.insert(arg, regex);},
            Err(error) => {panic!("{:?}", error);}
        }
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            for (name, regex) in regexes.iter() {
                let res = regex.search_str(&line, onig::options::ONIG_OPTION_NONE);
                match res {
                    Some(pos) => println!("{} => matched @ {}", name, pos),
                    None => println!("{} => did not match", name)
                }
            }
        }
    }
    println!("done");
}
