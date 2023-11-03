use crate::Solution;

type InputType = Vec<(i64, i64, i64)>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut t = 0;

    for (l, w, h) in input {
        t += 2 * l * w + 2 * w * h + 2 * h * l + (l * w * h / [l, w, h].iter().max().unwrap());
    }

    Solution::from(t)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut t = 0;

    for (l, w, h) in input {
        let mut temp = [l, w, h];
        temp.sort();
        t += l * w * h + 2 * temp[0] + 2 * temp[1];
    }

    Solution::from(t)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut dims: Vec<(i64, i64, i64)> = vec![];

    for l in input {
        let temp: Vec<i64> = l.split('x').map(|el| el.parse().unwrap()).collect();
        dims.push((temp[0], temp[1], temp[2]));
    }

    dims
}
