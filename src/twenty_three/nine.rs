use crate::{AdventOfCode, Day};

pub struct Nine;

impl Day for Nine {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "nine");
        let lines = content.lines();
        let mut histories = Vec::new();

        for line in lines {
            histories.push(Vec::from([line
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()]));
        }

        for history in histories.iter_mut() {
            let mut i = 0;
            let mut gen = gen_prev_history(&history[i]);
            while gen.iter().filter(|&x| *x != 0).count() > 0 {
                gen = gen_prev_history(&history[i]);
                history.push(gen.clone());
                i += 1;
            }
        }

        for history in histories.iter_mut() {
            let len = history.len() as i32;
            let mut val = 0;
            for (i, line) in history.clone().iter().rev().enumerate() {
                if let Some(ref mut next_line) = history.get_mut((len - (i as i32 + 2)) as usize) {
                    val = if val == 0 { line[line.len() - 1] } else { val }
                        + next_line[next_line.len() - 1];
                    next_line.push(val);
                }
            }
        }

        let mut r = 0;

        histories.iter().for_each(|h| {
            r += h[0].last().unwrap();
        });

        r.to_string()
    }

    fn two(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "nine");
        let lines = content.lines();
        let mut histories = Vec::new();

        for line in lines {
            histories.push(Vec::from([line
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()]));
        }

        for history in histories.iter_mut() {
            let mut i = 0;
            let mut gen = gen_prev_history(&history[i]);
            while gen.iter().filter(|&x| *x != 0).count() > 0 {
                gen = gen_prev_history(&history[i]);
                history.push(gen.clone());
                i += 1;
            }
        }

        for history in histories.iter_mut() {
            let len = history.len() as i32;
            let mut val = -1;
            for (i, line) in history.clone().iter().rev().enumerate() {
                if let Some(ref mut next_line) = history.get_mut((len - (i as i32 + 2)) as usize) {
                    val = next_line[0] - if val == -1 { line[0] } else { val };
                    // println!("{line:?} {next_line:?} {val}");
                    let mut values = [vec![val], next_line[..].to_vec()].concat();
                    next_line.clear();
                    next_line.append(&mut values);
                }
            }
        }

        let mut r = 0;

        histories.iter().for_each(|h| {
            // println!("v: {}", h[0].first().unwrap());
            r += h[0].first().unwrap();
        });

        r.to_string()
    }
}

fn gen_prev_history(v: &[i32]) -> Vec<i32> {
    let mut r = Vec::new();

    for i in 0..v.len() as i32 - 1 {
        r.push((v.get(&(i as usize) + 1).unwrap_or(&0)) - v[i as usize]);
    }

    r
}
