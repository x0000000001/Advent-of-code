use crate::Solution;

type InputType = Vec<Vec<i64>>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(
        input
            .into_iter()
            .map(|mut t| {
                t.sort();
                t
            })
            .filter(|t| t[0] + t[1] > t[2])
            .count() as i64,
    )
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);
    let mut nt: Vec<Vec<i64>> = vec![];

    for i in 0..input.len() / 3 {
        nt.push(Vec::from([
            input[3 * i][0],
            input[3 * i + 1][0],
            input[3 * i + 2][0],
        ]));
        nt.push(Vec::from([
            input[3 * i][1],
            input[3 * i + 1][1],
            input[3 * i + 2][1],
        ]));
        nt.push(Vec::from([
            input[3 * i][2],
            input[3 * i + 1][2],
            input[3 * i + 2][2],
        ]));
    }

    Solution::from(
        nt.into_iter()
            .map(|mut t| {
                t.sort();
                t
            })
            .filter(|t| t[0] + t[1] > t[2])
            .count() as i64,
    )
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for l in input {
        res.push(
            l.split_whitespace()
                .map(|el| el.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }

    res
}
