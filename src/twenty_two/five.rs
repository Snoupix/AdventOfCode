use std::collections::HashMap;

use crate::{AdventOfCode, Day};

pub struct Five;

impl Day for Five {
    fn one(testing: bool) -> String {
        let mut result: String = String::new();
        let mut contents = AdventOfCode::read_file_to_string(testing, "five");
        let mut actions = compute_file(contents); // move [0] from [1] to [2]
                                                  /* let mut state: HashMap<u16, Vec<&str>> = HashMap::from([
                                                      (1, Vec::from(["Z", "N"])),
                                                      (2, Vec::from(["M", "C", "D"])),
                                                      (3, Vec::from(["P"])),
                                                  ]); */
        let mut state: HashMap<u16, Vec<&str>> = HashMap::from([
            (1, Vec::from(["L", "N", "W", "T", "D"])),
            (2, Vec::from(["C", "P", "H"])),
            (3, Vec::from(["W", "P", "H", "N", "D", "G", "M", "J"])),
            (4, Vec::from(["C", "W", "S", "N", "T", "Q", "L"])),
            (5, Vec::from(["P", "H", "C", "N"])),
            (6, Vec::from(["T", "H", "N", "D", "M", "W", "Q", "B"])),
            (7, Vec::from(["M", "B", "R", "J", "G", "S", "L"])),
            (8, Vec::from(["Z", "N", "W", "G", "V", "B", "R", "T"])),
            (9, Vec::from(["W", "G", "D", "N", "P", "L"])),
        ]);

        for a in actions {
            for _ in 0..a[0] {
                let to_move = state
                    .get_mut(&a[1])
                    .expect(&format!("Cannot get container {:?}", a))
                    .pop()
                    .expect(&format!("Cannot get last element {:?} {:?}", a, state));
                state
                    .get_mut(&a[2])
                    .expect(&format!("Cannot get container {:?}", a))
                    .push(to_move);
            }
        }

        for i in 1..state.len() + 1 {
            result.push_str(
                state
                    .get(&(i as u16))
                    .expect("Cannot get state element")
                    .last()
                    .expect("Cannot get last element"),
            );
        }

        result
    }

    fn two(testing: bool) -> String {
        let mut result: String = String::new();
        let mut contents = AdventOfCode::read_file_to_string(testing, "five");
        let mut actions = compute_file(contents); // move [0] from [1] to [2]
        let mut state: HashMap<u16, Vec<&str>> = HashMap::from([
            (1, Vec::from(["L", "N", "W", "T", "D"])),
            (2, Vec::from(["C", "P", "H"])),
            (3, Vec::from(["W", "P", "H", "N", "D", "G", "M", "J"])),
            (4, Vec::from(["C", "W", "S", "N", "T", "Q", "L"])),
            (5, Vec::from(["P", "H", "C", "N"])),
            (6, Vec::from(["T", "H", "N", "D", "M", "W", "Q", "B"])),
            (7, Vec::from(["M", "B", "R", "J", "G", "S", "L"])),
            (8, Vec::from(["Z", "N", "W", "G", "V", "B", "R", "T"])),
            (9, Vec::from(["W", "G", "D", "N", "P", "L"])),
        ]);

        for a in actions {
            let mut container = state
                .get_mut(&a[1])
                .expect(&format!("Cannot get container {:?}", a));

            let mut to_move: Vec<&str> = Vec::new();

            for _ in 0..a[0] {
                to_move.push(container.pop().expect("Cannot pop container"));
            }

            to_move.reverse();

            for s in to_move {
                state
                    .get_mut(&a[2])
                    .expect(&format!("Cannot get container {:?}", a))
                    .push(s);
            }
        }

        for i in 1..state.len() + 1 {
            result.push_str(
                state
                    .get(&(i as u16))
                    .expect("Cannot get state element")
                    .last()
                    .expect("Cannot get last element"),
            );
        }

        result
    }
}

fn compute_file(ctnt: String) -> Vec<Vec<u16>> {
    let mut j = 0;
    let mut vector: Vec<Vec<u16>> = Vec::new();
    let s = ctnt
        .replace('\r', "")
        .replace('\n', ",")
        .replace("move ", "")
        .replace(" from ", "-")
        .replace(" to ", "-");

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b',' {
            vector.push(
                s[j..i]
                    .split('-')
                    .map(|nb| nb.parse::<u16>().expect("Failed to parse to u16"))
                    .collect::<Vec<u16>>(),
            );
            j = i + 1;
        }
    }

    vector.push(
        s[j..]
            .split('-')
            .map(|nb| nb.parse::<u16>().expect("Failed to parse to u16"))
            .collect::<Vec<u16>>(),
    );

    vector
}
