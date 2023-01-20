use crate::class::{AdventOfCode, Day};

pub struct Twelve;

impl Day for Twelve {
    fn one(testing: bool) -> String {
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "twelve");
        let input = compute_file(content);

        String::from("")
    }

    fn two(testing: bool) -> String {
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "twelve");
        let input = compute_file(content);

        String::from("")
    }
}

fn compute_file(ctnt: String) -> Vec<String> {
    Vec::new()
}
