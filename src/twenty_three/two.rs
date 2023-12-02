use regex::Regex;

use crate::{AdventOfCode, Day};

pub struct Two;

impl Day for Two {
    fn one(testing: bool) -> String {
        const RED_LIMIT: u32 = 12;
        const GREEN_LIMIT: u32 = 13;
        const BLUE_LIMIT: u32 = 14;
        let re_id: Regex = Regex::new(r"Game [0-9]+").unwrap();
        let re_red: Regex = Regex::new(r"[0-9]+ red").unwrap();
        let re_green: Regex = Regex::new(r"[0-9]+ green").unwrap();
        let re_blue: Regex = Regex::new(r"[0-9]+ blue").unwrap();

        let content = AdventOfCode::read_file_to_string(testing, "two");
        let v: Vec<&str> = content.lines().collect();
        let mut r = Vec::new();

        let get_nbr = |r: &Regex, s: &str| -> Vec<u32> {
            r.find_iter(s)
                .map(|m| {
                    m.as_str()
                        .replace(char::is_alphabetic, "")
                        .replace(char::is_whitespace, "")
                        .parse::<u32>()
                        .unwrap()
                })
                .collect()
        };

        for l in v {
            let id = get_nbr(&re_id, l)[0];
            let red = get_nbr(&re_red, l);
            let green = get_nbr(&re_green, l);
            let blue = get_nbr(&re_blue, l);

            if red.into_iter().filter(|&x| x > RED_LIMIT).count() == 0
                && green.into_iter().filter(|&x| x > GREEN_LIMIT).count() == 0
                && blue.into_iter().filter(|&x| x > BLUE_LIMIT).count() == 0
            {
                r.push(id);
            }
        }

        r.into_iter().reduce(|a, b| a + b).unwrap().to_string()
    }

    fn two(testing: bool) -> String {
        let re_id: Regex = Regex::new(r"Game [0-9]+").unwrap();
        let re_red: Regex = Regex::new(r"[0-9]+ red").unwrap();
        let re_green: Regex = Regex::new(r"[0-9]+ green").unwrap();
        let re_blue: Regex = Regex::new(r"[0-9]+ blue").unwrap();

        let content = AdventOfCode::read_file_to_string(testing, "two");
        let v: Vec<&str> = content.lines().collect();
        let mut r = Vec::new();

        let get_nbr = |r: &Regex, s: &str| -> Vec<u32> {
            r.find_iter(s)
                .map(|m| {
                    m.as_str()
                        .replace(char::is_alphabetic, "")
                        .replace(char::is_whitespace, "")
                        .parse::<u32>()
                        .unwrap()
                })
                .collect()
        };

        for l in v {
            let id = get_nbr(&re_id, l)[0];
            let mut red = get_nbr(&re_red, l);
            let mut green = get_nbr(&re_green, l);
            let mut blue = get_nbr(&re_blue, l);
            red.sort();
            green.sort();
            blue.sort();
            r.push(red[red.len() - 1] * green[green.len() - 1] * blue[blue.len() - 1])
        }

        r.into_iter().reduce(|a, b| a + b).unwrap().to_string()
    }
}
