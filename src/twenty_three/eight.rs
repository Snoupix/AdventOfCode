use std::collections::{BTreeMap, HashMap};

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

    fn two(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "eight");
        let lines = content.lines();
        let re_letters = Regex::new(r"[A-Z|0-9]+").unwrap();
        let mut r = 0;
        let mut steps = Vec::new();
        let mut nodes = HashMap::new();
        let mut map = BTreeMap::new();

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
                    let idx = letters[0];
                    let l_r = (letters[1], letters[2]);

                    if idx.ends_with('A') {
                        nodes.insert(idx, l_r);
                    }

                    map.insert(idx, l_r);
                }
            }
        }

        loop {
            let mut i = 0;

            for (n, (left, right)) in nodes.clone() {
                let curr_idx = match steps[r % steps.len()] {
                    'R' => right,
                    'L' => left,
                    x => panic!("Found {x} where it's either R or L"),
                };

                // println!(
                //     "{} Going through {n} with next idx as {curr_idx}",
                //     steps[r % steps.len()]
                // );

                nodes.insert(curr_idx, map[curr_idx]);
                nodes.remove(n);

                if curr_idx.ends_with('Z') {
                    i += 1;
                }
            }

            r += 1;

            if i == nodes.len() {
                break;
            }
        }

        // r > 82626

        r.to_string()
    }
}
