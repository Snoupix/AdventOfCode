use crate::{AdventOfCode, Day};

pub struct Eleven;

impl Day for Eleven {
    fn one(testing: bool) -> String {
        let mut content = AdventOfCode::read_file_to_string(testing, "eleven");
        let input = compute_file(content);

        String::from("")
    }

    fn two(testing: bool) -> String {
        let mut content = AdventOfCode::read_file_to_string(testing, "eleven");
        let input = compute_file(content);

        String::from("")
    }
}

fn compute_file(ctnt: String) -> Vec<String> {
    Vec::new()
}
