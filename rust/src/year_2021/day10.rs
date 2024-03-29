use crate::Solution;

fn line_corruption_score(line: &str) -> i64 {
    let chs = line.chars();
    let mut stack: Vec<char> = Vec::new();

    for c in chs {
        match c {
            '{' => {
                stack.push(c);
            }
            '[' => {
                stack.push(c);
            }
            '<' => {
                stack.push(c);
            }
            '(' => {
                stack.push(c);
            }
            '}' => match stack.pop() {
                None => return 1197,
                Some(previous) => match previous {
                    '{' => (),
                    _ => return 1197,
                },
            },
            ']' => match stack.pop() {
                None => return 57,
                Some(previous) => match previous {
                    '[' => (),
                    _ => return 57,
                },
            },
            '>' => match stack.pop() {
                None => return 25137,
                Some(previous) => match previous {
                    '<' => (),
                    _ => return 25137,
                },
            },
            ')' => match stack.pop() {
                None => return 3,
                Some(previous) => match previous {
                    '(' => (),
                    _ => return 3,
                },
            },

            _ => (),
        }
    }

    0
}

pub fn part1(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    Solution::from(
        input
            .iter()
            .map(|l| line_corruption_score(l) as i64)
            .sum::<i64>(),
    )
}

fn line_corruption_score2(line: &str) -> i64 {
    let chs = line.chars();
    let mut stack: Vec<char> = Vec::new();

    for c in chs {
        match c {
            '{' => {
                stack.push(c);
            }
            '[' => {
                stack.push(c);
            }
            '<' => {
                stack.push(c);
            }
            '(' => {
                stack.push(c);
            }
            '}' => match stack.pop() {
                None => return 0,
                Some(previous) => match previous {
                    '{' => (),
                    _ => return 0,
                },
            },
            ']' => match stack.pop() {
                None => return 0,
                Some(previous) => match previous {
                    '[' => (),
                    _ => return 0,
                },
            },
            '>' => match stack.pop() {
                None => return 0,
                Some(previous) => match previous {
                    '<' => (),
                    _ => return 0,
                },
            },
            ')' => match stack.pop() {
                None => return 0,
                Some(previous) => match previous {
                    '(' => (),
                    _ => return 0,
                },
            },

            _ => (),
        }
    }

    let mut score = 0;

    while !stack.is_empty() {
        match stack.pop().unwrap() {
            '{' => {
                score *= 5;
                score += 3;
            }
            '[' => {
                score *= 5;
                score += 2;
            }
            '<' => {
                score *= 5;
                score += 4;
            }
            '(' => {
                score *= 5;
                score += 1;
            }
            _ => (),
        }
    }

    score
}

pub fn part2(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let mut scores: Vec<i64> = input.iter().map(|l| line_corruption_score2(l)).collect();
    scores.retain(|v| *v != 0);
    scores.sort();

    Solution::from(scores[(scores.len() - 1) / 2])
}
