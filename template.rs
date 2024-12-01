use crate::{AdventOfCode, Day};

pub struct One;

impl Day for One {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "one");
        let lines = content.lines();

        "".into()
    }
}
