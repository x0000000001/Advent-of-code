use crate::Solution;

type InputType = (i64, i64);

pub fn part1(s: String) -> Solution {
    let (x, y) = parse(s);

    let mut code: i64 = 20151125;
    let factor = 252533;
    let modulo = 33554393;

    let n = ((x + y) * (x + y - 1)) / 2 - x + 1;

    for _ in 1..n {
        code *= factor;
        code %= modulo;
    }

    Solution::from(code)
}

pub fn part2(_: String) -> Solution {
    Solution::Num(0)
}

fn parse(s: String) -> InputType {
    let mut input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    input[0] = input[0].replace(",", "");
    input[0] = input[0].replace(".", "");
    let w = input[0].split_whitespace().collect::<Vec<&str>>();

    (
        w[w.len() - 3].parse().unwrap(),
        w.last().unwrap().parse().unwrap(),
    )
}
