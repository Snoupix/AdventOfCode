use std::{cmp::Ordering, str::Chars};

use crate::{AdventOfCode, Day};

pub struct One;

impl Day for One {
    fn one(testing: bool) -> String {
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "one");
        let mut j = 0;
        let mut vector = Vec::new();
        let s = content
            .replace('\r', "")
            .replace('\n', " ")
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_whitespace())
            .collect::<String>();

        for (i, &item) in s.as_bytes().iter().enumerate() {
            if item == b' ' {
                vector.push(s[j..i].parse::<u32>().unwrap());
                j = i + 1;
            }
        }

        for x in vector.iter_mut() {
            if *x < 10 && *x > 0 {
                *x *= 11;
            }

            if *x > 99 {
                let s = x.to_string();
                let mut _s = s.chars();
                *x = format!("{}{}", _s.nth(0).unwrap(), _s.last().unwrap())
                    .parse::<u32>()
                    .unwrap()
            }
        }

        vector.into_iter().reduce(|a, b| a + b).unwrap().to_string()
    }

    fn two(testing: bool) -> String {
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "one");
        let mut v = Vec::new();
        let mut j = 0;
        let nbrs = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .map(|&s| s.chars())
        .collect::<Vec<Chars>>();
        let _s = content.replace('\r', "").replace('\n', " ");
        let mut _s = _s.split(' ');
        let _ = _s.next_back();

        for s in _s {
            let mut _v = Vec::new();
            for (i, c) in s.chars().enumerate() {
                if c.is_ascii_digit() {
                    let mut j = i;
                    let mut chars = s[i..].chars();
                    for _c in chars {
                        if !_c.is_ascii_digit() {
                            break;
                        }

                        j += 1;
                    }

                    s[i..j]
                        .chars()
                        .for_each(|_c| _v.push(_c.to_string().parse::<u32>().unwrap()));
                    continue;
                }

                for _chars in &nbrs {
                    let mut j = i;
                    for (_i, _c) in _chars.clone().enumerate() {
                        if i + _i >= s.len() || s.chars().collect::<Vec<char>>()[_i + i] != _c {
                            break;
                        }

                        j += 1;
                    }

                    if j - i > 1 {
                        _v.push(match &s[i..j] {
                            "one" => 1,
                            "two" => 2,
                            "three" => 3,
                            "four" => 4,
                            "five" => 5,
                            "six" => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine" => 9,
                            _ => 0,
                        });

                        _v.retain(|x| *x != 0);
                    }
                }
            }

            match _v.len().partial_cmp(&2).unwrap() {
                Ordering::Less => _v.push(_v[0]),
                Ordering::Greater => {
                    let mut iter = _v.into_iter();
                    _v = Vec::from([iter.next().unwrap(), iter.last().unwrap()]);
                }
                _ => (),
            }

            v.push(format!("{}{}", _v[0], _v[1]).parse::<u32>().unwrap());
        }

        v.into_iter().reduce(|a, b| a + b).unwrap().to_string()
    }
}