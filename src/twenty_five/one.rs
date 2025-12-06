use crate::{AdventOfCode, Day};

pub struct One;

impl Day for One {
    type Output = i32;

    fn one(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "one");
        let lines = content.lines();
        let mut zeros: Self::Output = 0;
        let mut curr: Self::Output = 50;

        for line in lines {
            let mut chars = line.chars();
            let dir = chars.next().unwrap();
            let nb = chars.collect::<String>().parse::<Self::Output>().unwrap();

            match dir {
                'L' => {
                    curr = (curr - nb).rem_euclid(100);
                }
                'R' => {
                    curr = (curr + nb) % 100;
                }
                _ => unreachable!(),
            }

            if curr == 0 {
                zeros += 1;
            }
        }

        if testing {
            assert_eq!(zeros, 3);
        }

        // 1195
        zeros
    }

    fn two(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "one");
        let mut lines = content.lines();
        let mut zeros: i32 = 0;
        let mut curr: Self::Output = 50;

        for i in 0..lines.clone().count() {
            let mut chars = lines.next().unwrap().chars();
            let dir = chars.next().unwrap();
            let mut nb = chars.collect::<String>().parse::<Self::Output>().unwrap();

            let full_rot = nb / 100;

            if full_rot > 0 {
                zeros += full_rot;
                nb %= 100;
            }

            let was_zero = curr == 0;

            match dir {
                'L' => {
                    curr -= nb;
                }
                'R' => {
                    curr += nb;
                }
                _ => unreachable!(),
            }

            if curr == 0 {
                zeros += 1;
                continue;
            }

            if curr < 0 {
                curr += 100;

                if !was_zero {
                    zeros += 1;
                }
            }

            if curr > 99 {
                curr -= 100;
                zeros += 1;
            }
        }

        if testing {
            assert_eq!(zeros, 6);
        }

        // 6770
        zeros
    }
}
