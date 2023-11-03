use crate::Solution;

type InputType = String;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut i = 1;

    loop {
        let digest = md5::compute(format!("{}{}", input, i.to_string()));
        let h = format!("{:x}", digest);
        if h[0..5] == *"00000" {
            return Solution::from(i);
        }

        i += 1;
    }
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);
    let mut i = 1;

    loop {
        let digest = md5::compute(format!("{}{}", input, i.to_string()));
        let h = format!("{:x}", digest);
        if h[0..5] == *"00000" {
            return Solution::from(i);
        }

        i += 1;
    }
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    input[0].clone()
}
