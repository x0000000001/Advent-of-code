use crate::Solution;

type InputType = Vec<String>;

pub fn count_real_chars(s: &str) -> i64 {
    let mut i = 0;
    let mut count = 0;
    let len = s.len();

    let c = s.chars().collect::<Vec<char>>();

    while i < len {
        match c[i] {
            '\\' => match c[i + 1] {
                'x' => i += 4,
                _ => i += 2,
            },
            _ => i += 1,
        }

        count += 1;
    }

    count
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let real_chars_count: i64 = input
        .iter()
        .map(|l| count_real_chars(&l[1..(l.len() - 1)]))
        .sum();
    let writted_chars_count: i64 = input.iter().map(|l| l.chars().count() as i64).sum();

    Solution::from(writted_chars_count - real_chars_count)
}

pub fn count_added_chars(s: &str) -> i64 {
    let mut i = 0;
    let mut count = 0;
    let len = s.len();

    let c = s.chars().collect::<Vec<char>>();

    while i < len {
        match c[i] {
            '\\' => count += 2,
            '\"' => count += 2,
            _ => count += 1,
        }
        i += 1;
    }

    count + 2 // +2 for quotation marks
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let writted_chars_count: i64 = input.iter().map(|l| l.chars().count() as i64).sum();
    let added_chars_count: i64 = input.iter().map(|l| count_added_chars(&l)).sum();

    Solution::from(added_chars_count - writted_chars_count)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect()
}
