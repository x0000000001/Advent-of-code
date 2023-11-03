use crate::Solution;

type InputType = Vec<(char, i64)>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut orientation = 0;
    let mut position = (0, 0);

    for (d, i) in input {
        if d == 'R' {
            orientation = (orientation + 1) % 4;
        } else {
            orientation = (orientation - 1) % 4;
            if orientation < 0 {
                orientation = 3;
            }
        }

        match orientation {
            0 => position.1 += i,
            1 => position.0 += i,
            2 => position.1 -= i,
            3 => position.0 -= i,
            _ => (),
        }
    }

    Solution::from(position.0.abs() + position.1.abs())
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut orientation = 0;
    let mut position: (i64, i64) = (0, 0);
    let mut seen: Vec<(i64, i64)> = vec![];

    'main: for (d, i) in input {
        if d == 'R' {
            orientation = (orientation + 1) % 4;
        } else {
            orientation = (orientation - 1) % 4;
            if orientation < 0 {
                orientation = 3;
            }
        }

        let movement = match orientation {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            _ => (0, -1),
        };

        for _ in 0..i {
            position.0 += movement.0;
            position.1 += movement.1;
            if seen.contains(&position) {
                break 'main;
            } else {
                seen.push(position);
            }
        }
    }

    Solution::from(position.0.abs() + position.1.abs())
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();
    input[0]
        .replace(",", "")
        .split_whitespace()
        .map(|s| (s.chars().nth(0).unwrap(), s[1..].parse().unwrap()))
        .collect()
}
