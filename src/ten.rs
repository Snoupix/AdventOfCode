use crate::class::{AdventOfCode, Day};

pub struct Ten;

impl Day for Ten {
    fn one(testing: bool) -> String {
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "ten");
        let instructions = compute_file(content);
        let mut i_iter = instructions.iter();
        let mut signals: Vec<i64> = Vec::new();
        let mut register: i64 = 1;
        let mut cycles_skip = 0;
        let mut cycles = 0;
        let mut nb_to_add = 0;

        loop {
            cycles += 1;

            if cycles == 20 || cycles % 40 == 20 {
                signals.push(register * cycles);
                println!("{} {} {}", cycles, register, register * cycles);
            }

            if cycles_skip > 0 {
                cycles_skip -= 1;
                continue;
            }

            let next = i_iter.next();

            if next.is_none() {
                break;
            }

            println!("{} {} {:?}", register, nb_to_add, &next);
            register += nb_to_add;

            let i = next.unwrap();

            if i.contains("noop") {
                continue;
            }

            cycles_skip = 1;

            nb_to_add = i.split(' ').last().unwrap().parse::<i64>().unwrap();
        }

        let r = signals.iter().sum::<i64>();

        if testing {
            assert_eq!(13140, r);
        }

        r.to_string()
    }

    fn two(testing: bool) -> String {
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "ten");
        let input = compute_file(content);

        String::from("")
    }
}

fn compute_file(ctnt: String) -> Vec<String> {
    let mut j = 0;
    let mut vector: Vec<String> = Vec::new();
    let s = ctnt.replace('\r', "").replace('\n', "=");

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b'=' {
            vector.push(s[j..i].to_string());
            j = i + 1;
        }
    }

    vector.push(s[j..].to_string());

    vector
}
