// ownerproof-2415883-1669992207-9ee88f361b58
#![allow(unused)]

mod enums;

mod twenty_three;
mod twenty_two;

use std::{fs::File, io::Read, vec};

use clap::Parser;

macro_rules! use_year {
    ($year:ident) => {
        use crate::$year::*;

        println!(
            "{}",
            match stringify!($year) {
                "twenty_two" => "Using year 2022",
                "twenty_three" => "Using year 2023",
                _ => "Using unrecognized year",
            }
        );
    };
}

// https://docs.rs/quote/0.6.13/quote/macro.quote.html#constructing-identifiers
// macro_rules! nb_to_ident {
//     ($nb:expr) => {
//         m
//     };
// }

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    day: u32,

    #[clap(short, long)]
    sub: u32,

    #[clap(short, long)]
    test: Option<bool>,
}

pub struct AdventOfCode {
    current_day: enums::Day,
    current_sub_day: enums::SubDay,
    testing: bool,
}

pub trait Day {
    fn one(testing: bool) -> String;

    fn two(testing: bool) -> String;
}

#[derive(Debug)]
enum AOCArgError {
    Day,
    Sub,
}

use_year!(twenty_two);

fn main() {
    AdventOfCode::new(Args::parse()).run();
}

impl Default for AdventOfCode {
    fn default() -> Self {
        Self {
            current_day: enums::Day::One,
            current_sub_day: enums::SubDay::One,
            testing: false,
        }
    }
}

impl AdventOfCode {
    pub fn new(args: Args) -> Self {
        Self::parse_args(args)
            .map_err(|err| match err {
                AOCArgError::Day => "Wrong day number: Enter a day between 1 and 25",
                AOCArgError::Sub => "Wrong sub day number: Enter a sub day between 1 and 2",
            })
            .unwrap()
    }

    fn parse_args(args: Args) -> Result<AdventOfCode, AOCArgError> {
        let mut s = Self::default();

        if !(1..=25).contains(&args.day) {
            return Err(AOCArgError::Day);
        }

        if !(1..=2).contains(&args.sub) {
            return Err(AOCArgError::Sub);
        }

        if let Some(is_test) = args.test {
            s.testing = is_test;
        }

        s.current_day = match args.day {
            1 => enums::Day::One,
            2 => enums::Day::Two,
            3 => enums::Day::Three,
            4 => enums::Day::Four,
            5 => enums::Day::Five,
            6 => enums::Day::Six,
            7 => enums::Day::Seven,
            8 => enums::Day::Eight,
            9 => enums::Day::Nine,
            10 => enums::Day::Ten,
            11 => enums::Day::Eleven,
            12 => enums::Day::Twelve,
            13 => enums::Day::Thirteen,
            14 => enums::Day::Fourteen,
            15 => enums::Day::Fifteen,
            16 => enums::Day::Sixteen,
            17 => enums::Day::Seventeen,
            18 => enums::Day::Eighteen,
            19 => enums::Day::Nineteen,
            20 => enums::Day::Twenty,
            21 => enums::Day::Twentyone,
            22 => enums::Day::Twentytwo,
            23 => enums::Day::Twentythree,
            24 => enums::Day::Twentyfour,
            25 => enums::Day::Twentyfive,
            _ => return Err(AOCArgError::Day),
        };

        s.current_sub_day = match args.sub {
            1 => enums::SubDay::One,
            2 => enums::SubDay::Two,
            _ => return Err(AOCArgError::Sub),
        };

        Ok(s)
    }

    pub fn run(self) {
        println!(
            "Result {:?}",
            match self.current_day {
                enums::Day::One => {
                    match self.current_sub_day {
                        enums::SubDay::One => One::one(self.testing),
                        enums::SubDay::Two => One::two(self.testing),
                    }
                }
                enums::Day::Two => {
                    match self.current_sub_day {
                        enums::SubDay::One => Two::one(self.testing),
                        enums::SubDay::Two => Two::two(self.testing),
                    }
                }
                enums::Day::Three => {
                    match self.current_sub_day {
                        enums::SubDay::One => Three::one(self.testing),
                        enums::SubDay::Two => Three::two(self.testing),
                    }
                }
                enums::Day::Four => {
                    match self.current_sub_day {
                        enums::SubDay::One => Four::one(self.testing),
                        enums::SubDay::Two => Four::two(self.testing),
                    }
                }
                enums::Day::Five => {
                    match self.current_sub_day {
                        enums::SubDay::One => Five::one(self.testing),
                        enums::SubDay::Two => Five::two(self.testing),
                    }
                }
                enums::Day::Six => {
                    match self.current_sub_day {
                        enums::SubDay::One => Six::one(self.testing),
                        enums::SubDay::Two => Six::two(self.testing),
                    }
                }
                enums::Day::Seven => {
                    match self.current_sub_day {
                        enums::SubDay::One => Seven::one(self.testing),
                        enums::SubDay::Two => Seven::two(self.testing),
                    }
                }
                enums::Day::Eight => {
                    match self.current_sub_day {
                        enums::SubDay::One => Eight::one(self.testing),
                        enums::SubDay::Two => Eight::two(self.testing),
                    }
                }
                enums::Day::Nine => {
                    match self.current_sub_day {
                        enums::SubDay::One => Nine::one(self.testing),
                        enums::SubDay::Two => Nine::two(self.testing),
                    }
                }
                enums::Day::Ten => {
                    match self.current_sub_day {
                        enums::SubDay::One => Ten::one(self.testing),
                        enums::SubDay::Two => Ten::two(self.testing),
                    }
                }
                enums::Day::Eleven => {
                    match self.current_sub_day {
                        enums::SubDay::One => Eleven::one(self.testing),
                        enums::SubDay::Two => Eleven::two(self.testing),
                    }
                }
                enums::Day::Twelve => {
                    match self.current_sub_day {
                        enums::SubDay::One => Twelve::one(self.testing),
                        enums::SubDay::Two => Twelve::two(self.testing),
                    }
                }
                _ => String::from(""),
            }
        );
    }

    fn get_file_path(testing: bool, day: &str) -> String {
        let mut absolute_path = String::from("D:/Documents/taff/adventofcode/src/input/");

        if testing {
            absolute_path.push_str("example.txt");
        } else {
            absolute_path.push_str(&format!("input_{}.txt", day));
        }

        absolute_path
    }

    pub fn read_file_to_string(buf: &mut String, testing: bool, day: &str) {
        let path = Self::get_file_path(testing, day);
        let mut file = File::open(path).expect("Couldn't open input");
        file.read_to_string(buf).expect("Couldn't read input");
    }
}
