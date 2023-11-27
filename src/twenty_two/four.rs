use crate::{AdventOfCode, Day};

pub struct Four;

impl Day for Four {
    fn one(testing: bool) -> String {
        let mut result: u32 = 0;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "four");
        let mut vector = compute_file(content);

        for v in vector.iter_mut() {
            *v = (v[0]..=v[1]).map(u32::from).collect();
        }

        for i in 0..vector.len() {
            if i % 2 != 0 {
                continue;
            }
            if i + 1 == vector.len() {
                break;
            }

            let mut case1_fully_contains = true;
            let mut case2_fully_contains = true;
            let case1 = &vector[i];
            let case2 = &vector[i + 1];

            for nb in case1 {
                if let None = case2.iter().find(|n| n == &nb) {
                    case1_fully_contains = false;
                    break;
                }
            }

            if !case1_fully_contains {
                for nb in case2 {
                    if let None = case1.iter().find(|n| n == &nb) {
                        case2_fully_contains = false;
                        break;
                    }
                }
            }

            if case1_fully_contains || case2_fully_contains {
                result = result + 1;
            }
        }

        result.to_string()
    }

    fn two(testing: bool) -> String {
        let mut result: u32 = 0;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "four");
        let mut vector = compute_file(content);

        for v in vector.iter_mut() {
            *v = (v[0]..=v[1]).map(u32::from).collect();
        }

        for i in 0..vector.len() {
            if i % 2 != 0 {
                continue;
            }
            if i + 1 == vector.len() {
                break;
            }

            let mut case1_fully_contains = false;
            let mut case2_fully_contains = false;
            let case1 = &vector[i];
            let case2 = &vector[i + 1];

            for nb in case1 {
                if let Some(_) = case2.iter().find(|n| n == &nb) {
                    case1_fully_contains = true;
                    break;
                }
            }

            if !case1_fully_contains {
                for nb in case2 {
                    if let Some(_) = case1.iter().find(|n| n == &nb) {
                        case2_fully_contains = true;
                        break;
                    }
                }
            }

            if case1_fully_contains || case2_fully_contains {
                result = result + 1;
            }
        }

        result.to_string()
    }
}

fn compute_file(ctnt: String) -> Vec<Vec<u32>> {
    let mut j = 0;
    let mut vector: Vec<Vec<u32>> = Vec::new();
    let s = ctnt
        .replace('\r', "")
        .replace(['\n', ','], " ")
        .replace('-', ",");

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            vector.push(
                s[j..i]
                    .split(',')
                    .map(|nb| nb.parse::<u32>().expect("Failed to parse to u32"))
                    .collect::<Vec<u32>>(),
            );
            j = i + 1;
        }
    }

    vector.push(
        s[j..]
            .split(',')
            .map(|nb| nb.parse::<u32>().expect("Failed to parse to u32"))
            .collect::<Vec<u32>>(),
    );

    vector
}
