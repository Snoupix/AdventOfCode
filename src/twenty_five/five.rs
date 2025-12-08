use crate::{AdventOfCode, Day};

pub struct Five;

impl Day for Five {
    type Output = i64;

    fn one(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "five");
        let lines = content.lines();
        let mut are_ranges = true;
        let mut ranges = Vec::new();
        let mut res: Self::Output = 0;

        for line in lines {
            if line.trim().is_empty() {
                are_ranges = false;
                continue;
            }

            if are_ranges {
                let mut range = line.split('-');
                let begin: Self::Output = range.next().unwrap().parse().unwrap();
                let end: Self::Output = range.next().unwrap().parse().unwrap();

                assert!(begin <= end);

                ranges.push((begin, end));
            } else {
                let id = line.parse::<Self::Output>().unwrap();

                for &(low, high) in &ranges {
                    if id >= low && id <= high {
                        res += 1;
                        break;
                    }
                }
            }
        }

        if testing {
            assert_eq!(res, 3);
        }

        // 661
        res
    }

    fn two(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "five");
        let lines = content.lines();
        let mut ranges = Vec::new();

        for line in lines {
            if line.trim().is_empty() {
                break;
            }

            let mut range_parts = line.split('-');
            let begin: i64 = range_parts.next().unwrap().parse().unwrap();
            let end: i64 = range_parts.next().unwrap().parse().unwrap();

            assert!(begin <= end);

            ranges.push((begin, end));
        }

        ranges.sort_by_key(|k| k.0);

        let mut merged_ranges = Vec::from([ranges[0]]);

        for &range in &ranges[1..] {
            let last = merged_ranges.last_mut().unwrap();

            if range.0 <= last.1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        }

        let res = merged_ranges
            .iter()
            .map(|(start, end)| (end - start) + 1)
            .sum();

        if testing {
            assert_eq!(res, 14);
        }

        // 359526404143208
        res
    }
}
