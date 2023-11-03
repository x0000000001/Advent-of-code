use crate::Solution;

type InputType = u64;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let input = (input / 10) as usize;
    let mut sums = vec![0; input as usize];

    for i in 1..input {
        for j in 1..(input / i) {
            sums[i * j] += i;
        }
    }

    for i in 0..sums.len() {
        if sums[i] >= input {
            return Solution::from(i as i64);
        }
    }

    Solution::from(0)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let input = (input / 11) as usize;
    let mut sums = vec![0; input as usize];

    for i in 1..input {
        for j in 1..(input / i).min(51) {
            sums[i * j] += i;
        }
    }

    for i in 0..sums.len() {
        if sums[i] >= input {
            return Solution::from(i as i64);
        }
    }

    Solution::from(0)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .parse()
        .unwrap()
}
