use std::{env, fs::File, io::Read, vec};

use crate::{
    eight::Eight, eleven::Eleven, enums, five::Five, four::Four, nine::Nine, one::One,
    seven::Seven, six::Six, ten::Ten, three::Three, twelve::Twelve, two::Two,
};

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
enum AOCError {
    WrongFirstArg,
    WrongSecondArg,
}

impl AdventOfCode {
    pub fn new() -> Self {
        Self::get_args(env::args()).unwrap()
    }

    fn get_args(args: env::Args) -> Result<AdventOfCode, AOCError> {
        let mut curr_day = enums::Day::One;
        let mut curr_sub = enums::SubDay::One;
        let mut test = false;

        for (i, a) in args.enumerate() {
            let arg = &a[..];

            match i {
                1 => {
                    match arg {
                        "1" => curr_day = enums::Day::One,
                        "2" => curr_day = enums::Day::Two,
                        "3" => curr_day = enums::Day::Three,
                        "4" => curr_day = enums::Day::Four,
                        "5" => curr_day = enums::Day::Five,
                        "6" => curr_day = enums::Day::Six,
                        "7" => curr_day = enums::Day::Seven,
                        "8" => curr_day = enums::Day::Eight,
                        "9" => curr_day = enums::Day::Nine,
                        "10" => curr_day = enums::Day::Ten,
                        "11" => curr_day = enums::Day::Eleven,
                        "12" => curr_day = enums::Day::Twelve,
                        "13" => curr_day = enums::Day::Thirteen,
                        "14" => curr_day = enums::Day::Fourteen,
                        "15" => curr_day = enums::Day::Fifteen,
                        "16" => curr_day = enums::Day::Sixteen,
                        "17" => curr_day = enums::Day::Seventeen,
                        "18" => curr_day = enums::Day::Eighteen,
                        "19" => curr_day = enums::Day::Nineteen,
                        "20" => curr_day = enums::Day::Twenty,
                        "21" => curr_day = enums::Day::Twentyone,
                        "22" => curr_day = enums::Day::Twentytwo,
                        "23" => curr_day = enums::Day::Twentythree,
                        "24" => curr_day = enums::Day::Twentyfour,
                        "25" => curr_day = enums::Day::Twentyfive,
                        _ => return Err(AOCError::WrongFirstArg),
                    };
                }
                2 => {
                    match arg {
                        "1" => curr_sub = enums::SubDay::One,
                        "2" => curr_sub = enums::SubDay::Two,
                        _ => return Err(AOCError::WrongSecondArg),
                    };
                }
                3 => {
                    if arg == "test" {
                        test = true;
                    }
                }
                _ => (),
            };
        }

        Ok(Self {
            current_day: curr_day,
            current_sub_day: curr_sub,
            testing: test,
        })
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
