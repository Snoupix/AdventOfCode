use std::collections::HashMap;

use regex::Regex;

use crate::{AdventOfCode, Day};

pub struct Three;

impl Day for Three {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "three");
        let v: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
        let re_chars = Regex::new(r"[^a-zA-Z\d\n\s\.]+").unwrap();
        let mut r = Vec::new();

        for (x, line) in v.iter().enumerate() {
            let mut curr_nb = String::new();
            let mut is_part_nb = false;
            for (y, c) in line.iter().enumerate() {
                if !c.is_ascii_digit() {
                    is_part_nb = false;
                    curr_nb = String::new();
                    continue;
                }

                curr_nb.push(*c);

                if !get_neighbors(&v, &re_chars, x as _, y as _).is_empty() {
                    is_part_nb = true;
                }

                if (line.get(y + 1).is_none()
                    || line.get(y + 1).is_some_and(|ch| !ch.is_ascii_digit()))
                    && is_part_nb
                {
                    r.push(curr_nb.parse::<u64>().unwrap());
                }
            }
        }

        r.into_iter().reduce(|a, b| a + b).unwrap().to_string()
    }

    fn two(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "three");
        let v: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
        let mut r = Vec::new();
        let mut h: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

        for (x, line) in v.iter().enumerate() {
            let mut curr_nb = String::new();
            let mut curr_pos: Option<(u32, u32)> = None;
            let mut has_gear = false;
            for (y, c) in line.iter().enumerate() {
                if !c.is_ascii_digit() {
                    curr_nb = String::new();
                    curr_pos = None;
                    has_gear = false;
                    continue;
                }

                curr_nb.push(*c);

                let pos = get_gear_pos(&v, x as _, y as _);

                if pos.is_none() && curr_pos.is_none() {
                    continue;
                }

                has_gear = true;

                if let Some(p) = pos {
                    curr_pos = Some(p);
                }

                if (line.get(y + 1).is_none()
                    || line.get(y + 1).is_some_and(|ch| !ch.is_ascii_digit()))
                    && has_gear
                {
                    let pos = curr_pos.unwrap();
                    let nb = curr_nb.parse::<u32>().unwrap();
                    match h.get_mut(&pos) {
                        Some(v) => {
                            v.push(nb);
                        }
                        None => {
                            h.insert(pos, Vec::from([nb]));
                        }
                    }
                }
            }
        }

        for (k, v) in h {
            if v.len() == 2 {
                r.push(v[0] * v[1]);
            }
        }

        r.into_iter().reduce(|a, b| a + b).unwrap().to_string()
    }
}

fn get_neighbors(v: &[Vec<char>], regex: &Regex, x: i32, y: i32) -> Vec<char> {
    let mut r = Vec::new();

    for xx in [-1i32, 0, 1] {
        for yy in [-1i32, 0, 1] {
            if x + xx >= 0 && x + xx < v.len() as i32 && y + yy >= 0 && y + yy < v[0].len() as i32 {
                r.push(v[(x + xx) as usize][(y + yy) as usize]);
            }
        }
    }

    r.into_iter()
        .filter(|ch| regex.is_match(&ch.to_string()))
        .collect()
}

fn get_gear_pos(v: &[Vec<char>], x: i32, y: i32) -> Option<(u32, u32)> {
    for xx in [-1i32, 0, 1] {
        for yy in [-1i32, 0, 1] {
            if x + xx >= 0
                && x + xx < v.len() as i32
                && y + yy >= 0
                && y + yy < v[0].len() as i32
                && v[(x + xx) as usize][(y + yy) as usize] == '*'
            {
                return Some(((x + xx) as _, (y + yy) as _));
            }
        }
    }

    None
}
