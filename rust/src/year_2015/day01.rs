use crate::Solution;

type InputType = Vec<char>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut c = 0;

    for s in input {
        if s == '(' {
            c += 1;
        } else {
            c -= 1
        }
    }

    Solution::from(c)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut c = 0;
    let mut i = 0;

    for s in input {
        i += 1;
        if s == '(' {
            c += 1;
        } else {
            c -= 1
        }
        if c == -1 {
            return Solution::from(i);
        }
    }

    Solution::NotFound
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .next()
        .unwrap()
        .chars()
        .collect()
}
