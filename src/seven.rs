use std::collections::HashMap;

use regex::Regex;

use crate::class::{AdventOfCode, Day};

pub struct Seven;

impl Day for Seven {
    fn one(testing: bool) -> String {
        let mut result = 0i64;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "seven");
        let commands = compute_file(content);

        let mut dir_sizes: HashMap<String, i64> = HashMap::from([(String::from("/"), 0)]);
        let mut dir_includes: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_path: Vec<String> = Vec::from(["/".to_string()]);

        for c in commands {
            let mut args = c.split(' ');
            let current_absolute_path = get_path(&current_path);

            // input commands
            if c.starts_with('$') {
                let cmd = args.nth(1).unwrap();

                if cmd == "ls" {
                    continue;
                }

                if cmd == "cd" {
                    let path = args.last().unwrap().to_string();

                    if path == "/" {
                        current_path = Vec::from(["/".to_string()]);
                        continue;
                    }

                    if path == ".." {
                        current_path.pop();
                        continue;
                    }

                    current_path.push(path.clone());

                    continue;
                }

                continue;
            }

            // output directory
            if c.starts_with("dir") {
                let path = get_path(&current_path);

                if dir_sizes.get(&path).is_none() {
                    dir_sizes.insert(path.clone(), 0);
                }

                if let Some(val) = dir_includes.get(&path) {
                    let mut set = val.clone();
                    let mut dir_path = current_absolute_path;

                    dir_path.push('/');
                    dir_path.push_str(args.last().unwrap());
                    set.push(dir_path.replace("//", "/"));

                    dir_includes.insert(path, set);
                } else {
                    let mut curr_path = Vec::from([current_absolute_path]);
                    curr_path.push(args.last().unwrap().to_string());

                    dir_includes.insert(path.clone(), Vec::from([get_path(&curr_path)]));
                }

                continue;
            }

            // output file
            if Regex::new(r"([0-9])+").unwrap().is_match(&c) {
                let path = get_path(&current_path);

                if dir_sizes.get(&path).is_none() {
                    dir_sizes.insert(path.clone(), 0);
                }

                dir_sizes.insert(
                    path.clone(),
                    dir_sizes.get(&path).unwrap() + (args.next().unwrap().parse::<i64>().unwrap()),
                );

                continue;
            }
        }

        // add sub directories to parents
        for (dir, incl_dir) in dir_includes.clone().iter() {
            for (dir2, incl_dir2) in dir_includes.clone().iter() {
                if dir == dir2 {
                    continue;
                }

                if incl_dir.contains(dir2) {
                    let mut val = incl_dir.to_owned();

                    for v in incl_dir2 {
                        val.push(v.to_owned());
                    }

                    dir_includes.insert(dir.clone(), val);
                }
            }
        }

        // update directories sizes with their children
        for key in sorted_hash_keys(&dir_includes) {
            let incl_dir = dir_includes.get(&key).unwrap();
            for d in incl_dir {
                //println!("-------");
                //println!("{:?} {:?}\n{} {}\n{:?} {:?}", dir_sizes.get(dir), dir_sizes.get(d), dir, d, dir_includes.get(dir), dir_includes.get(d));
                dir_sizes.insert(
                    key.clone(),
                    dir_sizes.get(&key).unwrap() + dir_sizes.get(d).unwrap(),
                );
            }
        }

        //println!("{:?} {:?} {:?}", dir_sizes, dir_includes, current_path);

        // problem is => the dir sizes sum is not right because if a directory is executed, the parent will calculate his own children even if the child has been done yet.

        for (_, size) in dir_sizes.iter() {
            if *size <= 100000 {
                result += size;
            }
        }

        result.to_string()
    }
    // 1844187
    // 2 => 1306611 == r https://github.com/aldanor/aoc-2022/blob/main/src/day07/mod.rs

    fn two(testing: bool) -> String {
        let (total, required) = (70000000, 30000000);
        let mut result = 0i64;
        let mut content = String::new();
        AdventOfCode::read_file_to_string(&mut content, testing, "seven");
        let commands = compute_file(content);

        let mut dir_sizes: HashMap<String, i64> = HashMap::from([(String::from("/"), 0)]);
        let mut dir_includes: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_path: Vec<String> = Vec::from(["/".to_string()]);

        for c in commands {
            let mut args = c.split(' ');
            let current_absolute_path = get_path(&current_path);

            // input commands
            if c.starts_with('$') {
                let cmd = args.nth(1).unwrap();

                if cmd == "ls" {
                    continue;
                }

                if cmd == "cd" {
                    let path = args.last().unwrap().to_string();

                    if path == "/" {
                        current_path = Vec::from(["/".to_string()]);
                        continue;
                    }

                    if path == ".." {
                        current_path.pop();
                        continue;
                    }

                    current_path.push(path.clone());

                    continue;
                }

                continue;
            }

            // output directory
            if c.starts_with("dir") {
                let path = get_path(&current_path);

                if dir_sizes.get(&path).is_none() {
                    dir_sizes.insert(path.clone(), 0);
                }

                if let Some(val) = dir_includes.get(&path) {
                    let mut set = val.clone();
                    let mut dir_path = current_absolute_path;

                    dir_path.push('/');
                    dir_path.push_str(args.last().unwrap());
                    set.push(dir_path.replace("//", "/"));

                    dir_includes.insert(path, set);
                } else {
                    let mut curr_path = Vec::from([current_absolute_path]);
                    curr_path.push(args.last().unwrap().to_string());

                    dir_includes.insert(path.clone(), Vec::from([get_path(&curr_path)]));
                }

                continue;
            }

            // output file
            if Regex::new(r"([0-9])+").unwrap().is_match(&c) {
                let path = get_path(&current_path);

                if dir_sizes.get(&path).is_none() {
                    dir_sizes.insert(path.clone(), 0);
                }

                dir_sizes.insert(
                    path.clone(),
                    dir_sizes.get(&path).unwrap() + (args.next().unwrap().parse::<i64>().unwrap()),
                );

                continue;
            }
        }

        // add sub directories to parents
        for key in sorted_hash_keys(&dir_includes) {
            for (dir, incl_dir) in dir_includes.clone().iter() {
                if &key == dir {
                    continue;
                }

                let dir_incl = dir_includes.get(&key).unwrap();

                if dir_incl.contains(&key) {
                    let mut val = dir_incl.to_owned();

                    for v in incl_dir {
                        val.push(v.to_owned());
                    }

                    dir_includes.insert(dir.clone(), val);
                }
            }
        }
        /* for (dir, incl_dir) in dir_includes.clone().iter() {
            for (dir2, incl_dir2) in dir_includes.clone().iter() {
                if dir == dir2 { continue }

                if incl_dir.contains(dir2) {
                    let mut val = incl_dir.to_owned();

                    for v in incl_dir2 {
                        val.push(v.to_owned());
                    }

                    dir_includes.insert(dir.clone(), val);
                }
            }
        } */

        // update directories sizes with their children
        for key in sorted_hash_keys(&dir_includes) {
            let incl_dir = dir_includes.get(&key).unwrap();

            for d in incl_dir {
                dir_sizes.insert(
                    key.clone(),
                    dir_sizes.get(&key).unwrap() + dir_sizes.get(d).unwrap(),
                );
            }
        }

        /* println!(
            "root: {:?} required: {:?} unused: {:?} so at least: {:?}",
            *dir_sizes.get("/").unwrap(),
            required,
            total - *dir_sizes.get("/").unwrap(),
            required - (total - *dir_sizes.get("/").unwrap())
        ); */

        dir_sizes
            .iter()
            .map(|(_, &size)| size)
            .collect::<Vec<i64>>()
            .iter()
            .filter(|&s| *s + (total - *dir_sizes.get("/").unwrap()) >= required)
            .min()
            .unwrap()
            .to_string()
    }
}

fn get_path(path: &[String]) -> String {
    path.join("/").replace("//", "/")
}

fn sorted_hash_keys(h: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut v: Vec<String> = h.iter().map(|(s, _)| s.to_owned()).collect();

    v.sort_by_key(|b| std::cmp::Reverse(b.len()));
    //v.sort_by(|a, b| b.len().cmp(&a.len()));

    v
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
