use crate::{AdventOfCode, Day};

pub struct One;

impl Day for One {
    fn one(testing: bool) -> String {
        let mut result: u32 = 0;
        let mut content = AdventOfCode::read_file_to_string(testing, "one");
        let vector = compute_file(content);

        for vec in vector.iter() {
            let mut cal: u32 = 0;

            for n in vec {
                cal = cal + n;
            }

            if cal > result {
                result = cal;
            }
        }

        result.to_string()
    }

    fn two(testing: bool) -> String {
        let mut results: Vec<u32> = Vec::new();
        let mut content = AdventOfCode::read_file_to_string(testing, "one");
        let vector = compute_file(content);

        for vec in vector.iter() {
            let mut cal: u32 = 0;

            for n in vec {
                cal = cal + n;
            }

            results.push(cal);
        }

        results.sort();

        (results[results.len() - 1] + results[results.len() - 2] + results[results.len() - 3])
            .to_string()
    }
}

fn compute_file(ctnt: String) -> Vec<Vec<u32>> {
    let mut j = 0;
    let mut vector: Vec<Vec<u32>> = Vec::new();
    let s = ctnt
        .replace('\r', "")
        .replace("\n\n", ", ")
        .replace('\n', ",")
        .replace(", ", " ");

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            vector.push(
                s[j..i]
                    .split(',')
                    .map(|n| n.parse::<u32>().expect(&format!("Can't parse '{}'", n)[..]))
                    .collect(),
            );
            j = i + 1;
        }
    }

    vector
}
