use std::collections::HashMap;

use crate::Solution;

type InputType = u64;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut circle_length = 8;
    let mut sum_circles = 1;
    let mut i = 1;

    loop {
        if sum_circles + circle_length < input {
            sum_circles += circle_length;
        } else {
            break;
        }

        circle_length += 8;
        i += 1;
    }

    let remainder = input - sum_circles - 1;

    let mut x: i64 = i;
    let mut y: i64 = -i + 1;

    for _ in 0..remainder {
        if x == i && y < i {
            y += 1;
        } else if y == i && x > -i {
            x -= 1;
        } else if x == -i && y > -i {
            y -= 1;
        } else {
            x += 1;
        }
    }

    Solution::from(x.abs() + y.abs())
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut vals: HashMap<(i64, i64), u64> = HashMap::new();
    vals.insert((0, 0), 1);

    let mut x = 1;
    let mut y = 0;
    let mut index = 1;

    loop {
        let mut sum = 0;

        for i in x - 1..x + 2 {
            for j in y - 1..y + 2 {
                sum += vals.get(&(i, j)).unwrap_or(&0);
            }
        }

        if sum > input {
            return Solution::from(sum as i64);
        }

        vals.insert((x, y), sum);

        if y == -index && x == -index {
            x += 1;
            index += 1;
        } else if x == index && y < index {
            y += 1;
        } else if y == index && x > -index {
            x -= 1;
        } else if x == -index && y > -index {
            y -= 1;
        } else {
            x += 1;
        }
    }
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .parse()
        .unwrap()
}
