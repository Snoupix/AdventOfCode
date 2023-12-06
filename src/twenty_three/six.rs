use crate::{AdventOfCode, Day};

pub struct Six;

impl Day for Six {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "six");
        let numbers: Vec<Vec<u32>> = parse_file(content);
        let time = numbers[0].clone();
        let distance = numbers[1].clone();
        let mut r = Vec::new();

        for i in 0..time.len() {
            let mut _r = 0;

            for holding in 0..time[i] {
                let speed = holding * (time[i] - holding);

                if speed > distance[i] {
                    _r += 1;
                }
            }

            r.push(_r);
        }

        r.into_iter().reduce(|a, b| a * b).unwrap().to_string()
    }

    fn two(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "six");
        let numbers = content
            .lines()
            .map(|l| {
                l.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let time = numbers[0];
        let distance = numbers[1];
        let mut r = Vec::new();
        let mut _r = 0;

        for holding in 0..time {
            let speed = holding * (time - holding);

            if speed > distance {
                _r += 1;
            }
        }

        r.push(_r);

        r.into_iter().reduce(|a, b| a * b).unwrap().to_string()
    }
}

fn parse_file(content: String) -> Vec<Vec<u32>> {
    let mut v = Vec::new();

    for (i, line) in content.lines().enumerate() {
        for n in line
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
            .collect::<String>()
            .trim()
            .split(' ')
        {
            if n.trim() == "" {
                continue;
            }
            if v.get(i).is_none() {
                v.push(Vec::new());
            }

            v[i].push(n.parse::<u32>().unwrap());
        }
    }

    v
}
