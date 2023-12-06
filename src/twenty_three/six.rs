use crate::{AdventOfCode, Day};

pub struct Six;

impl Day for Six {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "five");
        let lines: Vec<String> = Self::parse_file(content);

        String::new()
    }
}
