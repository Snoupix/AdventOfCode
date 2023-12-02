use std::collections::HashMap;

use crate::{AdventOfCode, Day};

pub struct Three;

impl Day for Three {
    fn one(testing: bool) -> String {
        let mut result: u16 = 0;

        let mut aplha: HashMap<String, u16> = HashMap::new();
        let low_alpha: Vec<&str> = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];

        for i in 0..low_alpha.len() {
            aplha.insert(low_alpha[i].to_string(), (i + 1) as u16);
        }

        for i in 0..low_alpha.len() {
            aplha.insert(
                low_alpha[i].to_string().to_uppercase(),
                (i + 1 + low_alpha.len()) as u16,
            );
        }

        let mut content = AdventOfCode::read_file_to_string(testing, "three");
        let rucksack = compute_file(content);

        for items in &rucksack {
            let half = items.len() / 2;
            let first_half = &items[0..half];
            let second_half = &items[half..];

            for &letter in first_half.as_bytes().iter() {
                if let Some(i) = second_half.find(letter as char) {
                    result = result + aplha.get(second_half.get(i..i + 1).unwrap()).unwrap();
                    break;
                }
            }
        }

        result.to_string()
    }

    fn two(testing: bool) -> String {
        let mut result: u16 = 0;

        let mut aplha: HashMap<String, u16> = HashMap::new();
        let low_alpha: Vec<&str> = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];

        for i in 0..low_alpha.len() {
            aplha.insert(low_alpha[i].to_string(), (i + 1) as u16);
        }

        for i in 0..low_alpha.len() {
            aplha.insert(
                low_alpha[i].to_string().to_uppercase(),
                (i + 1 + low_alpha.len()) as u16,
            );
        }

        let mut content = AdventOfCode::read_file_to_string(testing, "three");
        let mut rucksack: Vec<Vec<String>> = Vec::new();

        let mut j = 0;
        for (i, line) in compute_file(content).iter().enumerate() {
            if i % 3 == 0 {
                rucksack.push(Vec::from([line.to_owned()]));

                if i != 0 {
                    j = j + 1;
                }
                continue;
            }

            rucksack[j].push(line.to_owned());
        }

        for elf_bag in rucksack {
            for &letter in elf_bag[0].as_bytes().iter() {
                let index = match elf_bag[1].find(letter as char) {
                    Some(i) => i,
                    None => continue,
                };
                match elf_bag[2].find(letter as char) {
                    Some(i) => (),
                    None => continue,
                };

                result = result
                    + aplha
                        .get(elf_bag[1].get(index..index + 1).unwrap())
                        .unwrap();
                break;
            }
        }

        result.to_string()
    }
}

fn compute_file(ctnt: String) -> Vec<String> {
    let mut j = 0;
    let mut vector: Vec<String> = Vec::new();
    let s = ctnt.replace('\r', "").replace('\n', " ");

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            vector.push(s[j..i].to_string());
            j = i + 1;
        }
    }

    vector.push(s[j..].to_string());

    vector
}
