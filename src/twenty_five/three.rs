use crate::{AdventOfCode, Day};

pub struct Three;

impl Day for Three {
    type Output = u64;

    fn one(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "three");
        let lines = content.lines();
        let mut res: Self::Output = 0;

        for line in lines {
            let vec: Vec<Self::Output> = line
                .chars()
                .map(|c| (c as Self::Output) - ('0' as Self::Output))
                .collect();

            res += build_nbrs(vec);
        }

        if testing {
            assert_eq!(res, 357);
        }

        // 16858
        res
    }

    fn two(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "three");
        let lines = content.lines();
        let mut res: Self::Output = 0;

        for line in lines {
            let vec: Vec<Self::Output> = line
                .chars()
                .map(|c| (c as Self::Output) - ('0' as Self::Output))
                .collect();

            res += build_nbrs_2(vec);
        }

        if testing {
            assert_eq!(res, 3121910778619);
        }

        // 167549941654721
        res
    }
}

fn build_nbrs(vec: Vec<u64>) -> u64 {
    let mut nb = 0;
    let mut cursor_offset = isize::MAX;

    while cursor_offset != 0 {
        let first = vec.iter().enumerate().fold((-1, 0), |(_i, acc), (i, &nb)| {
            let i = i as isize;

            if nb > acc && i < cursor_offset {
                (i, nb)
            } else {
                (_i, acc)
            }
        });

        let second = vec.iter().enumerate().fold((-1, 0), |(_i, acc), (i, &nb)| {
            let i = i as isize;

            if nb > acc && i > first.0 {
                (i, nb)
            } else {
                (_i, acc)
            }
        });

        if first.0 != -1 && second.0 != -1 {
            nb = nb.max(format!("{}{}", first.1, second.1).parse().unwrap());
        }
        cursor_offset = first.0;
    }

    nb
}

fn build_nbrs_2(vec: Vec<u64>) -> u64 {
    let len = vec.len();

    let mut cursor = len - 12;
    let mut stack: Vec<u64> = Vec::with_capacity(len);

    for digit in vec {
        while !stack.is_empty() && cursor > 0 && digit > *stack.last().unwrap() {
            stack.pop();
            cursor -= 1;
        }

        stack.push(digit);
    }

    while cursor > 0 {
        stack.pop();
        cursor -= 1;
    }

    while stack.len() > 12 {
        stack.pop();
    }

    stack
        .into_iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}
