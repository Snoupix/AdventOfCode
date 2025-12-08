use crate::{AdventOfCode, Day};

pub struct Four;

impl Day for Four {
    type Output = u32;

    fn one(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "four");
        let lines = content.lines();
        let mut matrix = Vec::new();
        let mut res: Self::Output = 0;

        for (i, line) in lines.enumerate() {
            matrix.push(Vec::new());

            for c in line.chars() {
                matrix[i].push(c);
            }

            if i != 0 {
                assert!(matrix[i - 1].len() == matrix[i].len());
            }
        }

        for (x, row) in matrix.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if *col == '@' {
                    let neighbors = get_neighbors_count(&matrix, x, y) as Self::Output;

                    if neighbors < 4 {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    fn two(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "four");
        let lines = content.lines();
        let mut matrix = Vec::new();
        let mut res: Self::Output = 0;

        for (i, line) in lines.enumerate() {
            matrix.push(Vec::new());

            for c in line.chars() {
                matrix[i].push(c);
            }

            if i != 0 {
                assert!(matrix[i - 1].len() == matrix[i].len());
            }
        }

        let mut can_remove = true;
        while can_remove {
            let matrix_it = matrix.clone();

            can_remove = false;

            for (x, row) in matrix_it.iter().enumerate() {
                for (y, col) in row.iter().enumerate() {
                    if *col == '@' {
                        let neighbors = get_neighbors_count(&matrix, x, y) as Self::Output;

                        if neighbors < 4 {
                            can_remove = true;
                            res += 1;
                            matrix[x][y] = '.';
                        }
                    }
                }
            }
        }

        res
    }
}

fn get_neighbors_count(matrix: &[Vec<char>], x: usize, y: usize) -> u8 {
    let mut res = 0;

    for i in -1..=1isize {
        for j in -1..=1isize {
            if i == 0 && j == 0 {
                continue;
            }

            let x = (x as isize + i);
            let y = (y as isize + j);

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if matrix.get(x).is_some_and(|row| row.get(y).is_some()) && matrix[x][y] == '@' {
                res += 1
            }
        }
    }

    res
}
