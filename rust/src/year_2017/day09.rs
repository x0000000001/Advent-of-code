use crate::Solution;

type InputType = String;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut total_score = 0;
    let mut current_score = 0;
    let mut in_garbage = false;
    let mut skip_next = false;

    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if in_garbage {
            match c {
                '>' => in_garbage = false,
                '!' => skip_next = true,
                _ => (),
            }
        } else {
            match c {
                '<' => in_garbage = true,
                '{' => current_score += 1,
                '}' => {
                    total_score += current_score;
                    current_score -= 1;
                }
                _ => (),
            }
        }
    }

    Solution::from(total_score)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut in_garbage = false;
    let mut skip_next = false;
    let mut count = 0;

    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if in_garbage {
            match c {
                '>' => in_garbage = false,
                '!' => skip_next = true,
                _ => count += 1,
            }
        } else {
            match c {
                '<' => in_garbage = true,
                _ => (),
            }
        }
    }

    Solution::from(count)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()
        .pop()
        .unwrap()
}
