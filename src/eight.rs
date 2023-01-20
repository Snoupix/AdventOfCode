use crate::class::{AdventOfCode, Day};

pub struct Eight;

impl Day for Eight {
    fn one(testing: bool) -> String {
        let mut result = 0u16;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "eight");
        let map = compute_file(content);

        let test_edge_trees: Vec<u8> = "3037322623935390"
            .split("")
            .map(|n| {
                if n.is_empty() {
                    99
                } else {
                    n.parse::<u8>().unwrap()
                }
            })
            .filter(|n| n != &99)
            .collect();
        let mut test_edge_trees_iter = test_edge_trees.iter();

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if x == 0 || x == map[y].len() - 1 || y == 0 || y == map.len() - 1 {
                    if testing {
                        assert_eq!(test_edge_trees_iter.next(), Some(&map[y][x]));
                    }
                    result += 1;
                    continue;
                }

                if is_visible(&map, y, x) {
                    result += 1;
                }
            }
        }

        if testing {
            assert_eq!(result, 21);
        }

        result.to_string()
    }

    fn two(testing: bool) -> String {
        let mut result = 0u32;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "eight");
        let map = compute_file(content);

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let score = get_scenic_score(&map, y, x);

                //println!("{:?} {y} {x}", map[y][x]);
                //println!("{y} {x} {:?}", &score);

                if score > result {
                    result = score;
                }
            }
        }

        if testing {
            assert_eq!(result, 8);
        }

        result.to_string()
    }
}

// check left right top bottom
fn is_visible(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let tree = map[y][x];
    let mut index = y as i16;

    while index != -1 {
        if index == 0 {
            if map[index as usize][x] >= tree {
                break;
            }

            return true;
        }

        index -= 1;

        if map[index as usize][x] >= tree {
            break;
        }
    }

    index = y as i16;

    while index as usize != map.len() {
        if index as usize == map.len() - 1 {
            if map[index as usize][x] >= tree {
                break;
            }

            return true;
        }

        index += 1;

        if map[index as usize][x] >= tree {
            break;
        }
    }

    index = x as i16;

    while index != -1 {
        if index == 0 {
            if map[y][index as usize] >= tree {
                break;
            }

            return true;
        }

        index -= 1;

        if map[y][index as usize] >= tree {
            break;
        }
    }

    index = x as i16;

    while index as usize != map.len() {
        if index as usize == map[y].len() - 1 {
            if map[y][index as usize] >= tree {
                break;
            }

            return true;
        }

        index += 1;

        if map[y][index as usize] >= tree {
            break;
        }
    }

    false
}

fn get_scenic_score(map: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let mut left = 0u32;
    let mut right = 0u32;
    let mut top = 0u32;
    let mut bottom = 0u32;

    let tree = map[y][x];
    let mut index = x as i16;

    while index != -1 {
        if index == 0 {
            break;
        }

        index -= 1;
        left += 1;

        if map[y][index as usize] >= tree {
            break;
        }
    }

    index = x as i16;

    while index as usize != map[y].len() {
        if index as usize == map[y].len() - 1 {
            break;
        }

        index += 1;
        right += 1;

        if map[y][index as usize] >= tree {
            break;
        }
    }

    index = y as i16;

    while index != -1 {
        if index == 0 {
            break;
        }

        index -= 1;
        top += 1;

        if map[index as usize][x] >= tree {
            break;
        }
    }

    index = y as i16;

    while index as usize != map[y].len() {
        if index as usize == map[y].len() - 1 {
            break;
        }

        index += 1;
        bottom += 1;

        if map[index as usize][x] >= tree {
            break;
        }
    }

    left * right * top * bottom
}

fn compute_file(ctnt: String) -> Vec<Vec<u8>> {
    let mut j = 0;
    let mut vector: Vec<Vec<u8>> = Vec::new();
    let s = ctnt.replace('\r', "").replace('\n', ",");

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b',' {
            vector.push(
                s[j..i]
                    .split("")
                    .map(|n| {
                        if n.is_empty() {
                            return 99;
                        }
                        n.parse::<u8>().expect(&format!("Can't parse '{}'", n)[..])
                    })
                    .filter(|n| n != &99)
                    .collect::<Vec<u8>>(),
            );
            j = i + 1;
        }
    }

    vector.push(
        s[j..]
            .split("")
            .map(|n| {
                if n.is_empty() {
                    return 99;
                }
                n.parse::<u8>().expect(&format!("Can't parse '{}'", n)[..])
            })
            .filter(|n| n != &99)
            .collect::<Vec<u8>>(),
    );

    vector
}
