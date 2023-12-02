use crate::{AdventOfCode, Day};

pub struct Six;

impl Day for Six {
    fn one(testing: bool) -> String {
        let mut result: usize = 0;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "six");

        for (i, &s) in content.as_bytes().iter().enumerate() {
            let mut chars = Vec::new();

            for j in 0..4 {
                chars.push(get_char_index_string(&content, i + j));
            }

            if !has_duplicates(chars) {
                result = i + 4;
                break;
            }
        }

        result.to_string()
    }

    fn two(testing: bool) -> String {
        let mut result: usize = 0;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "six");

        for (i, &s) in content.as_bytes().iter().enumerate() {
            let mut chars = Vec::new();

            for j in 0..14 {
                chars.push(get_char_index_string(&content, i + j));
            }

            if !has_duplicates(chars) {
                result = i + 14;
                break;
            }
        }

        result.to_string()
    }
}

fn get_char_index_string(s: &String, i: usize) -> char {
    s.as_bytes()[i] as char
}

fn has_duplicates(chars: Vec<char>) -> bool {
    for i in 0..chars.len() {
        for j in 0..chars.len() {
            if j == i {
                continue;
            }

            if chars[i] == chars[j] {
                return true;
            }
        }
    }

    false
}
