use crate::Solution;

type InputType = Vec<String>;

fn is_valid(s: &str) -> bool {
    let w = s.split_whitespace().collect::<Vec<&str>>();
    for i in 0..w.len() {
        for j in 0..w.len() {
            if i == j {
                continue;
            }

            if w[i] == w[j] {
                return false;
            }
        }
    }

    true
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(input.into_iter().filter(|l| is_valid(l)).count() as i64)
}

fn are_anagrams(s0: &str, s1: &str) -> bool {
    if s0.len() != s1.len() {
        return false;
    }

    let c0 = s0.chars().collect::<Vec<char>>();
    let mut c1 = s1.chars().collect::<Vec<char>>();

    for i in 0..c0.len() {
        let mut was_found = false;

        for j in i..c1.len() {
            if c1[j] == c0[i] {
                c1.swap(i, j);
                was_found = true;
                break;
            }
        }

        if !was_found {
            return false;
        }
    }

    true
}

fn is_valid2(s: &str) -> bool {
    let w = s.split_whitespace().collect::<Vec<&str>>();
    for i in 0..w.len() {
        for j in 0..w.len() {
            if i == j {
                continue;
            }

            if are_anagrams(w[i], w[j]) {
                return false;
            }
        }
    }

    true
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(input.into_iter().filter(|l| is_valid2(l)).count() as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect()
}
