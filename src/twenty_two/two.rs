use std::collections::HashMap;

use crate::{AdventOfCode, Day};

#[derive(PartialEq)]
enum RPS {
    ROCK,
    PAPER,
    SCISSORS,
}

const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSE: u8 = 0;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

pub struct Two;

impl Day for Two {
    fn one(testing: bool) -> String {
        let mut result: u32 = 0;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "two");
        let games = compute_file(content);

        for g in games {
            result = result + fight(g[0].clone(), g[1].clone());
        }

        result.to_string()
    }

    fn two(testing: bool) -> String {
        let mut result: u32 = 0;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "two");
        let games = compute_file(content);

        for g in games {
            result = result + fight_two(g[0].clone(), g[1].clone());
        }

        result.to_string()
    }
}

fn fight(elf: String, player: String) -> u32 {
    let values: HashMap<String, RPS> = HashMap::from([
        (String::from("A"), RPS::ROCK),
        (String::from("B"), RPS::PAPER),
        (String::from("C"), RPS::SCISSORS),
        (String::from("X"), RPS::ROCK),
        (String::from("Y"), RPS::PAPER),
        (String::from("Z"), RPS::SCISSORS),
    ]);

    let e = values.get(&elf).unwrap();
    let p = values.get(&player).unwrap();

    let points = match p {
        RPS::ROCK => ROCK,
        RPS::PAPER => PAPER,
        RPS::SCISSORS => SCISSORS,
    };

    let win_type = match e {
        RPS::ROCK => match p {
            RPS::ROCK => DRAW,
            RPS::PAPER => WIN,
            RPS::SCISSORS => LOSE,
        },
        RPS::PAPER => match p {
            RPS::ROCK => LOSE,
            RPS::PAPER => DRAW,
            RPS::SCISSORS => WIN,
        },
        RPS::SCISSORS => match p {
            RPS::ROCK => WIN,
            RPS::PAPER => LOSE,
            RPS::SCISSORS => DRAW,
        },
    };

    (points + win_type).into()
}

fn fight_two(elf: String, player: String) -> u32 {
    let values: HashMap<String, RPS> = HashMap::from([
        (String::from("A"), RPS::ROCK),
        (String::from("B"), RPS::PAPER),
        (String::from("C"), RPS::SCISSORS),
        (String::from("X"), RPS::ROCK),
        (String::from("Y"), RPS::PAPER),
        (String::from("Z"), RPS::SCISSORS),
    ]);

    let e = values.get(&elf).unwrap();
    let p = values.get(&player).unwrap();

    match e {
        RPS::ROCK => match p {
            RPS::ROCK => LOSE + SCISSORS,
            RPS::PAPER => DRAW + ROCK,
            RPS::SCISSORS => WIN + PAPER,
        },
        RPS::PAPER => match p {
            RPS::ROCK => LOSE + ROCK,
            RPS::PAPER => DRAW + PAPER,
            RPS::SCISSORS => WIN + SCISSORS,
        },
        RPS::SCISSORS => match p {
            RPS::ROCK => LOSE + PAPER,
            RPS::PAPER => DRAW + SCISSORS,
            RPS::SCISSORS => WIN + ROCK,
        },
    }
    .into()
}

fn compute_file(ctnt: String) -> Vec<Vec<String>> {
    let mut j = 0;
    let mut vector: Vec<Vec<String>> = Vec::new();
    let s = ctnt
        .replace('\r', "")
        .replace("\n\n", ", ")
        .replace(' ', ",")
        .replace('\n', " ");

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            vector.push(
                s[j..i]
                    .split('-')
                    .map(|letter| letter.to_owned())
                    .collect::<Vec<String>>(),
            );
            j = i + 1;
        }
    }

    vector.push(
        s[j..]
            .split(',')
            .map(|letter| letter.to_owned())
            .collect::<Vec<String>>(),
    );

    vector
}
