use regex::Regex;

use crate::{AdventOfCode, Day};

pub struct Two;

impl Day for Two {
    type Output = u64;

    fn one(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "two");
        let lines = content.split(',');
        let mut res = 0;

        for line in lines {
            let range: Vec<Self::Output> =
                line.split('-').map(|s| s.trim().parse().unwrap()).collect();
            assert!(range.len() == 2);

            let begin = range[0];
            let end = range[1];
            assert!(begin < end);

            for id in begin..=end {
                let s = id.to_string();
                let mid = s.len() / 2;

                if s[..mid] == s[mid..] {
                    eprintln!("In {range:?}: {id}");
                    res += id;
                }
            }
        }

        if testing {
            assert_eq!(res, 1227775554);
        }

        // 24043483400
        res
    }

    fn two(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "two");
        let lines = content.split(',');
        let mut res = 0;

        for line in lines {
            let range: Vec<Self::Output> =
                line.split('-').map(|s| s.trim().parse().unwrap()).collect();
            assert!(range.len() == 2);

            let begin = range[0];
            let end = range[1];
            assert!(begin < end);

            for id in begin..=end {
                if has_common_pattern(id.to_string()) {
                    eprintln!(" - {range:?}: {id}");
                    res += id;
                }
            }
        }

        if testing {
            assert_eq!(res, 4174379265);
        }

        // 38262920235
        res
    }
}

fn has_common_pattern(s: String) -> bool {
    let mut pat = String::new();

    'outer: for (i, c) in s.char_indices() {
        let slice = &s[i..];

        // let re = Regex::new(&format!("({pat})")).unwrap();

        let (mut count, mut cursor) = (0, 0);
        if !pat.is_empty() {
            // My while loop is extreeeeemly more efficient than the Regex
            while cursor < slice.len() {
                let Some(offset) = slice[cursor..].find(&pat).map(|idx| idx + pat.len()) else {
                    pat.push(c);

                    continue 'outer;
                };

                cursor += offset;
                count += 1;
            }
        }

        if !pat.is_empty()
            && !slice.starts_with('0')
            // && pat.len() * re.find_iter(slice).count() == slice.len()
            && pat.len() * count == slice.len()
            && slice.contains(&pat)
        {
            eprint!("{slice}");
            return true;
        }

        pat.push(c);
    }

    false
}
