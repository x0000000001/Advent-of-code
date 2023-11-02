use crate::Solution;

type InputType = (u64, u64);

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut a = input.0;
    let mut b = input.1;
    let mut score = 0;

    for _ in 0..40000000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        if a % 65536 == b % 65536 {
            score += 1;
        }
    }

    Solution::from(score)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut a = input.0;
    let mut b = input.1;
    let mut score = 0;

    for _ in 0..5000000 {
        loop {
            a = (a * 16807) % 2147483647;

            if a % 4 == 0 {
                break;
            }
        }

        loop {
            b = (b * 48271) % 2147483647;

            if b % 8 == 0 {
                break;
            }
        }

        if a % 65536 == b % 65536 {
            score += 1;
        }
    }

    Solution::from(score)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    fn get_val(s: &str) -> u64 {
        let w = s.split_whitespace();
        w.last().unwrap().parse().unwrap()
    }

    (get_val(&input[0]), get_val(&input[1]))
}
