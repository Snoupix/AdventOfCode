use crate::{AdventOfCode, Day};

pub struct Ten;

impl Day for Ten {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "ten");
        let lines = content.lines();
        let mut r = 0;

        r.to_string()
    }
}
