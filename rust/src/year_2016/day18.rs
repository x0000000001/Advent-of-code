use crate::Solution;

type InputType = Vec<bool>;

fn count_safe_tiles(mut row: InputType, rows_count: usize) -> u64 {
    let mut safe_tiles_count = 0;

    for _ in 0..rows_count {
        safe_tiles_count += row.iter().filter(|b| !*b).count();

        let mut temp = vec![];

        for i in 0..row.len() {
            let previous = [
                if i > 0 { row[i - 1] } else { false },
                row[i],
                if i + 1 < row.len() { row[i + 1] } else { false },
            ];

            temp.push(match previous {
                [true, true, false] => true,
                [false, true, true] => true,
                [true, false, false] => true,
                [false, false, true] => true,
                _ => false,
            });
        }

        row.clear();
        row.append(&mut temp);
    }

    safe_tiles_count as u64
}

pub fn part1(s: String) -> Solution {
    let row = parse(s);

    Solution::from(count_safe_tiles(row, 40) as i64)
}

pub fn part2(s: String) -> Solution {
    let row = parse(s);

    Solution::from(count_safe_tiles(row, 400000) as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res = vec![];

    for c in input[0].chars() {
        res.push(match c {
            '.' => false,
            '^' => true,
            _ => panic!(),
        });
    }

    res
}
