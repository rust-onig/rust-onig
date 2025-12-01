use onig::RegSet;

fn main() {
    // Create a RegSet with multiple patterns
    let patterns = &[
        r"(\d{4})-(\d{2})-(\d{2})", // Date format: YYYY-MM-DD
        r"([a-zA-Z]+)@([^@\s]+)",   // Email format
        r"([a-z]+)",                // Lowercase words
    ];

    let regset = RegSet::new(patterns).expect("Failed to create RegSet");

    let test_strings = &["2023-12-25", "user@example.com", "hello", "no match here!"];

    // Find which pattern matches first
    println!("Basic matching:");
    for text in test_strings {
        match regset.find(text) {
            Some((pattern_idx, pos)) => {
                println!(
                    "  '{}' matches pattern {} at position {}",
                    text, pattern_idx, pos
                );
            }
            None => println!("  '{}' has no match", text),
        }
    }

    // Extract capture groups
    println!("\nCapture groups:");
    for text in test_strings {
        if let Some((pattern_idx, captures)) = regset.captures(text) {
            match pattern_idx {
                0 => println!(
                    "  Date: year={}, month={}, day={}",
                    captures.at(1).unwrap(),
                    captures.at(2).unwrap(),
                    captures.at(3).unwrap()
                ),
                1 => println!(
                    "  Email: user={}, domain={}",
                    captures.at(1).unwrap(),
                    captures.at(2).unwrap()
                ),
                2 => println!("  Word: {}", captures.at(1).unwrap()),
                _ => unreachable!(),
            }
        }
    }
}
