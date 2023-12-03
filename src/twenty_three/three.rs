use crate::{AdventOfCode, Day};

pub struct Three;

impl Day for Three {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "two");
        let v: Vec<&str> = content.lines().collect();

        String::new()
    }

    fn two(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "two");
        let v: Vec<&str> = content.lines().collect();

        String::new()
    }
}
