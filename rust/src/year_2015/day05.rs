use crate::Solution;

type InputType = Vec<String>;

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN_COUPLES: &[(char, char)] = &[('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

fn is_string_nice(s: &String) -> bool {
    if s.chars()
        .map(|e| if VOWELS.contains(&e) { 1 } else { 0 })
        .sum::<i32>()
        < 3
    {
        return false;
    }

    let mut appears_twice = false;

    let c = s.chars().collect::<Vec<char>>();
    for i in 0..(c.len() - 1) {
        if c[i] == c[i + 1] {
            appears_twice = true;
        }

        if FORBIDDEN_COUPLES.contains(&(c[i], c[i + 1])) {
            return false;
        }
    }

    if !appears_twice {
        return false;
    }

    true
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(
        input
            .into_iter()
            .map(|s| if is_string_nice(&s) { 1 } else { 0 })
            .sum::<i64>(),
    )
}

fn is_string_nice1(s: &String) -> bool {
    let mut has_double_pair = false;

    let c = s.chars().collect::<Vec<char>>();
    for i in 0..(c.len() - 1) {
        let couple = (c[i], c[i + 1]);
        for j in i + 2..(c.len() - 1) {
            if (c[j], c[j + 1]) == couple {
                has_double_pair = true;
                break;
            }
        }
    }

    if !has_double_pair {
        return false;
    }

    let mut appears_twice = false;

    let c = s.chars().collect::<Vec<char>>();
    for i in 0..(c.len() - 2) {
        if c[i] == c[i + 2] {
            appears_twice = true;
            break;
        }
    }

    if !appears_twice {
        return false;
    }

    true
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(
        input
            .into_iter()
            .map(|s| if is_string_nice1(&s) { 1 } else { 0 })
            .sum::<i64>(),
    )
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect()
}
