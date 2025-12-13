use crate::{AdventOfCode, Day};

pub struct Six;

impl Day for Six {
    type Output = i64;

    fn one(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "six");
        let lines = content.lines();
        let mut rows: Vec<Vec<Self::Output>> = Vec::new();
        let mut ops: Vec<&str> = Vec::new();
        let mut res = 0;

        for (i, line) in lines.enumerate() {
            let row = line.trim();

            for (j, v) in row.split_whitespace().enumerate() {
                if ["*", "+"].contains(&v) {
                    ops.push(v);
                    continue;
                }

                let v = v.parse().unwrap();

                if i == 0 {
                    rows.push(Vec::from([v]));
                    continue;
                }

                rows[j].push(v);
            }
        }

        assert_eq!(rows.len(), ops.len());

        for (row, op) in rows.into_iter().zip(ops.into_iter()) {
            res += match op {
                "*" => row.into_iter().product::<Self::Output>(),
                "+" => row.into_iter().sum::<Self::Output>(),
                c => panic!("Unexpected operator: {c}"),
            };
        }

        if testing {
            assert_eq!(res, 4277556);
        }

        // 5335495999141
        res
    }

    fn two(testing: bool) -> Self::Output {
        let content = AdventOfCode::read_file_to_string(testing, "six");
        let lines = content.lines();
        let mut rows: Vec<Vec<Vec<char>>> = Vec::new();
        let mut ops: Vec<&str> = Vec::new();
        let mut res = 0;

        // on each line/row:
        //      - check next spaceS; and for each space found:
        //          - check ever line/row at that index and if space, continue, else, goto next
        //          space above
        //      if space found, line[last i..curr offset - 1] is the nb
        //      else, last nb so line[last i..];

        for (row_nb, row) in lines.clone().enumerate() {
            if row.trim().starts_with(['*', '+']) {
                for op in row.split_whitespace() {
                    ops.push(op);
                }
                continue;
            }

            let mut i = 0;
            let mut base_i = i;
            let mut col_i = 0;

            dbg!(
                &row[i..],
                row[i..]
                    .find(|c: char| c.is_ascii_whitespace())
                    .map(|ix| &row[i..ix])
            );

            'outer: while let Some(idx) = row[i..].find(|c: char| c.is_ascii_whitespace()) {
                if idx == 0 {
                    i += 1;
                    continue;
                }

                let offset = i + idx;

                // dbg!(i, idx, base_i, &row[base_i..offset]);

                i = offset;

                for line in lines.clone() {
                    let c = line.chars().nth(offset).unwrap();

                    eprintln!("{:?} {} {} {} {:?} {}", &row[i..], i, idx, offset, &line, c);

                    if c != ' ' {
                        continue 'outer;
                    }
                }

                let nb = &row[base_i..offset];
                let chars = nb.chars().collect();

                base_i = i;
                col_i += 1;

                if row_nb == 0 {
                    rows.push(Vec::from([chars]));
                    continue;
                }

                rows[col_i].push(chars);
            }

            /*let indexes = row
                .match_indices(|c: char| c.is_numeric() || c == '\n')
                .collect::<Vec<_>>();

            assert_eq!(indexes.len(), row.split_whitespace().count());

            for (j, (offset, s)) in indexes.into_iter().enumerate() {
                let offset = if s == "\n" { offset } else { offset - 1 };
                let nb = row[i..offset].chars().collect::<Vec<char>>();

                if i == 0 {
                    rows.push(Vec::from([nb]));
                } else {
                    rows[j].push(nb);
                }

                i = offset;
            } */

            // (curr i || 0) + i of (next nb - 1) or \n
            //
        }

        dbg!(&rows);

        /*assert_eq!(rows.len(), ops.len());

        for op in ops {
            res += match op {
                "*" => row.into_iter().product::<Self::Output>(),
                "+" => row.into_iter().sum::<Self::Output>(),
                c => panic!("Unexpected operator: {c}"),
            };
        }*/

        if testing {
            assert_eq!(res, 3263827);
        }

        res
    }
}
