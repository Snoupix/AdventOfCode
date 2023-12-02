use std::collections::HashMap;

use crate::{AdventOfCode, Day};

pub struct Nine;

#[derive(Clone, Debug, Hash)]
struct Pos<T> {
    x: T,
    y: T,
}

impl PartialEq for Pos<usize> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Pos<usize> {}

impl Day for Nine {
    fn one(testing: bool) -> String {
        let mut content = AdventOfCode::read_file_to_string(testing, "nine");
        let movements = compute_file(content);
        let mut map: Vec<Vec<i32>> = create_map(20, 20);
        let mut visited_idxs: HashMap<Pos<usize>, bool> = HashMap::new();

        if !testing {
            map = create_map(500, 500);
        }

        let mut head_pos: Pos<usize> = Pos {
            x: map[0].len() / 2,
            y: map.len() / 2,
        };
        let mut tail_pos: Pos<usize> = Pos {
            x: map[0].len() / 2,
            y: map.len() / 2,
        };

        if testing {
            head_pos = Pos {
                x: map[0].len() / 2,
                y: map.len() / 2,
            };
            tail_pos = Pos {
                x: map[0].len() / 2,
                y: map.len() / 2,
            };
        }

        for m in movements {
            let mut mov = m.split(' ');
            let (direction, times) = (mov.nth(0).unwrap(), mov.last().unwrap());

            match direction {
                "R" => {
                    for _ in 0..times.parse::<usize>().unwrap() {
                        head_pos.x += 1;

                        for pos in follow_head(&map, &head_pos, &mut tail_pos) {
                            visited_idxs.insert(pos, true);
                        }

                        if testing {
                            print_map(&map, &head_pos, &tail_pos, &visited_idxs);
                        }
                    }
                }
                "L" => {
                    for _ in 0..times.parse::<usize>().unwrap() {
                        head_pos.x -= 1;

                        for pos in follow_head(&map, &head_pos, &mut tail_pos) {
                            visited_idxs.insert(pos, true);
                        }

                        if testing {
                            print_map(&map, &head_pos, &tail_pos, &visited_idxs);
                        }
                    }
                }
                "D" => {
                    for _ in 0..times.parse::<usize>().unwrap() {
                        head_pos.y += 1;

                        for pos in follow_head(&map, &head_pos, &mut tail_pos) {
                            visited_idxs.insert(pos, true);
                        }

                        if testing {
                            print_map(&map, &head_pos, &tail_pos, &visited_idxs);
                        }
                    }
                }
                "U" => {
                    for _ in 0..times.parse::<usize>().unwrap() {
                        head_pos.y -= 1;

                        for pos in follow_head(&map, &head_pos, &mut tail_pos) {
                            visited_idxs.insert(pos, true);
                        }

                        if testing {
                            print_map(&map, &head_pos, &tail_pos, &visited_idxs);
                        }
                    }
                }
                _ => panic!("Pattern not found {}", direction),
            }

            // println!("{:?} {:?}", head_index, map[head_index.y][head_index.x]);
        }

        visited_idxs.len().to_string()
    }
    // 6265 < r

    fn two(testing: bool) -> String {
        let mut content = AdventOfCode::read_file_to_string(testing, "nine");
        let games = compute_file(content);

        String::from("")
    }
}

// Create 2D array
fn create_map(x: i32, y: i32) -> Vec<Vec<i32>> {
    (0..x).map(|_| (0..y).collect::<Vec<i32>>()).collect()
}

fn follow_head(map: &Vec<Vec<i32>>, head: &Pos<usize>, tail: &mut Pos<usize>) -> Vec<Pos<usize>> {
    let mut visited: Vec<Pos<usize>> = Vec::from([tail.clone()]);
    let head_pos = map[head.y][head.x];

    while !is_around(head, tail) {
        if head.x == tail.x {
            if (head.y as i32 - tail.y as i32) < 0 {
                tail.y -= 1;
                continue;
            } else {
                tail.y += 1;
                continue;
            }
        }

        if head.y == tail.y {
            if (head.x as i32 - tail.x as i32) < 0 {
                tail.x -= 1;
                continue;
            } else {
                tail.x += 1;
                continue;
            }
        }

        let x_diff = (head.x as i32 - tail.x as i32).abs();
        let y_diff = (head.y as i32 - tail.y as i32).abs();

        if x_diff > 1 || y_diff > 1 {
            if (head.y as i32 - tail.y as i32) < 0 {
                tail.y -= 1;
            } else {
                tail.y += 1;
            }

            if (head.x as i32 - tail.x as i32) < 0 {
                tail.x -= 1;
            } else {
                tail.x += 1;
            }

            continue;
        }

        if (head.x as i32 - tail.x as i32) > 1 {
            tail.x -= 1;
        }

        if (head.y as i32 - tail.y as i32) > 1 {
            tail.y -= 1;
        }
    }

    visited
}

fn is_around(head: &Pos<usize>, tail: &Pos<usize>) -> bool {
    if (head.x as i32 - tail.x as i32).abs() <= 1 && (head.y as i32 - tail.y as i32).abs() <= 1 {
        return true;
    }

    false
}

fn print_map(
    map: &Vec<Vec<i32>>,
    head: &Pos<usize>,
    tail: &Pos<usize>,
    visited_idxs: &HashMap<Pos<usize>, bool>,
) {
    for y in 0..map.len() {
        println!(
            "{:?}",
            map[y]
                .iter()
                .enumerate()
                .map(|(i, val)| {
                    let pos = Pos {
                        x: *val as usize,
                        y,
                    };

                    if &pos == head {
                        return "H";
                    }

                    if &pos == tail {
                        return "T";
                    }

                    if visited_idxs.get(&pos).is_none() || *visited_idxs.get(&pos).unwrap() == false
                    {
                        "."
                    } else {
                        "#"
                    }
                })
                .collect::<Vec<&str>>()
                .join("")
        );
    }

    println!("");
}

fn compute_file(ctnt: String) -> Vec<String> {
    let mut j = 0;
    let mut vector: Vec<String> = Vec::new();
    let s = ctnt.replace('\r', "").replace('\n', "-");

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b'-' {
            vector.push(s[j..i].to_string());
            j = i + 1;
        }
    }

    vector.push(s[j..].to_string());

    vector
}
