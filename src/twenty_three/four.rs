use regex::Regex;

use std::collections::HashSet;

use crate::{AdventOfCode, Day};

pub struct Four;

impl Day for Four {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "four");
        let input: Vec<&str> = content.lines().collect();
        let re_card_id = Regex::new(r"Card\s+\d+").unwrap();
        let re_numbers = Regex::new(r"[|\d]+").unwrap();
        let mut r = 0;

        for line in input {
            let id = re_card_id
                .find(line)
                .unwrap()
                .as_str()
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let mut left_nbrs = Vec::new();
            let mut right_nbrs = Vec::new();
            let mut change_side = false;
            re_numbers.find_iter(line).skip(1).for_each(|m| {
                let s = m.as_str();
                if let Ok(n) = s.parse::<u64>() {
                    if !change_side {
                        left_nbrs.push(n);
                    } else {
                        right_nbrs.push(n);
                    }
                } else {
                    change_side = true;
                }
            });

            let values = left_nbrs
                .into_iter()
                .filter(|n| right_nbrs.contains(n))
                .collect::<Vec<_>>();

            if !values.is_empty() {
                r += 1 << (values.len() - 1);
            }
        }

        r.to_string()
    }

    fn two(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "four");
        let input: Vec<&str> = content.lines().collect();
        let re_card_id = Regex::new(r"Card\s+\d+").unwrap();
        let re_numbers = Regex::new(r"[|\d]+").unwrap();
        let mut card_count = vec![1; input.len()];
        let mut r = 0;

        for (idx, line) in input.iter().enumerate() {
            let id = re_card_id
                .find(line)
                .unwrap()
                .as_str()
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            let mut left_nbrs = Vec::new();
            let mut right_nbrs = Vec::new();
            let mut change_side = false;
            re_numbers.find_iter(line).skip(1).for_each(|m| {
                let s = m.as_str();
                if let Ok(n) = s.parse::<u32>() {
                    if !change_side {
                        left_nbrs.push(n);
                    } else {
                        right_nbrs.push(n);
                    }
                } else {
                    change_side = true;
                }
            });

            let values = left_nbrs
                .into_iter()
                .filter(|n| right_nbrs.contains(n))
                .collect::<Vec<_>>();

            if !values.is_empty() {
                let stop = std::cmp::min(idx + values.len(), card_count.len() - 1);

                for _ in 0..card_count[idx] {
                    (idx + 1..=stop).for_each(|i| card_count[i] += 1);
                }
            }

            r += card_count[idx];
        }

        r.to_string()
    }
}
