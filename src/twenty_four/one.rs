use std::collections::HashMap;

use crate::{AdventOfCode, Day};

pub struct One;

impl Day for One {
    type Output = u64;

    fn one(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "one");
        let mut lines = content.lines();
        let count = lines.clone().count();
        let mut list1 = Vec::with_capacity(count);
        let mut list2 = Vec::with_capacity(count);
        let mut res = 0;

        for line in lines {
            let nbrs: Vec<Self::Output> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            assert_eq!(nbrs.len(), 2);

            list1.push(nbrs[0]);
            list2.push(nbrs[1]);
        }

        list1.sort();
        list2.sort();

        assert_eq!(list1.len(), list2.len());

        for i in 0..list1.len() {
            let x = list1[i];
            let y = list2[i];
            res += Self::Output::abs_diff(x, y);
        }

        res
    }

    fn two(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "one");
        let mut lines = content.lines();
        let count = lines.clone().count();
        let mut list1 = Vec::with_capacity(count);
        let mut hash = HashMap::with_capacity(count);
        let mut res = 0;

        for line in lines {
            let nbrs: Vec<Self::Output> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            assert_eq!(nbrs.len(), 2);

            list1.push(nbrs[0]);

            hash.entry(nbrs[1]).and_modify(|n| *n += 1).or_insert(1);
        }

        for n in list1 {
            res += n * hash.get(&n).unwrap_or(&0);
        }

        res
    }
}
