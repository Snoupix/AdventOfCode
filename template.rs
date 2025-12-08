use crate::{AdventOfCode, Day};

pub struct One;

impl Day for One {
    type Output = u32;

    fn one(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "one");
        let lines = content.lines();
        let mut res = 0;

        if testing {
            assert_eq!(res, 0);
        }

        res
    }
}
