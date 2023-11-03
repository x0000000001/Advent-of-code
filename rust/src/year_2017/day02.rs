use crate::Solution;

type InputType = Vec<Vec<i64>>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut sum = 0;

    for l in input {
        sum += l.iter().max().unwrap() - l.iter().min().unwrap();
    }

    Solution::from(sum)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut sum = 0;

    'line_loop: for l in input {
        for i in 0..l.len() {
            for j in 0..l.len() {
                if i == j {
                    continue;
                }
                if l[i] % l[j] == 0 {
                    sum += l[i] / l[j];
                    continue 'line_loop;
                }
            }
        }
    }

    Solution::from(sum)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![vec![]; input.len()];

    for i in 0..input.len() {
        res[i] = input[i]
            .split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect();
    }

    res
}
