use std::collections::BTreeMap;

use regex::Regex;

use crate::{AdventOfCode, Day};

pub struct Eight;

impl Day for Eight {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "eight");
        let lines = content.lines();
        let re_letters = Regex::new(r"[A-Z]+").unwrap();
        let mut r = 0;
        let mut steps = Vec::new();
        let mut map = BTreeMap::new();
        let mut curr_idx = "AAA";

        for l in lines {
            let letters = re_letters
                .find_iter(l)
                .map(|m| m.as_str())
                .collect::<Vec<_>>();

            match letters.len() {
                0 => continue,
                1 => {
                    steps = letters[0].chars().collect();
                }
                _ => {
                    map.insert(letters[0], (letters[1], letters[2]));
                }
            }
        }

        loop {
            if curr_idx == "ZZZ" {
                break;
            }

            curr_idx = match steps[r % steps.len()] {
                'R' => map[curr_idx].1,
                'L' => map[curr_idx].0,
                x => panic!("Found {x} where it's either R or L"),
            };

            r += 1;
        }

        r.to_string()
    }
}
