use crate::Solution;

type InputType = String;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn next(s: &mut String, index: usize) {
    if s.len() < index + 1 {
        return;
    }

    let last_char = s.chars().nth(index).unwrap();
    let next_char =
        ASCII_LOWER[(ASCII_LOWER.iter().position(|el| *el == last_char).unwrap() + 1) % 26];
    s.replace_range(index..(index + 1), &next_char.to_string());
    if next_char == 'a' && index > 0 {
        next(s, index - 1);
    }
}

pub fn is_valid(s: &String) -> bool {
    let chars = s.chars().collect::<Vec<char>>();

    // first condition
    let mut has_met = false;
    let indices = chars
        .iter()
        .map(|el| ASCII_LOWER.iter().position(|c| c == el).unwrap())
        .collect::<Vec<usize>>();

    for i in 0..(chars.len() - 2) {
        if indices[i + 1] == indices[i] + 1 && indices[i + 2] == indices[i] + 2 {
            has_met = true;
            break;
        }
    }

    if !has_met {
        return false;
    }

    // second condition
    for c in chars.iter() {
        if ['i', 'o', 'l'].contains(&c) {
            return false;
        }
    }

    // third condition
    let mut count = 0;
    let mut previous_char = None;

    let mut i = 0;
    while i < (chars.len() - 1) {
        if previous_char != Some(chars[i]) && chars[i] == chars[i + 1] {
            previous_char = Some(chars[i]);
            count += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    if count < 2 {
        return false;
    }

    true
}

pub fn next_password(mut s: String) -> String {
    let len = s.len() - 1;
    next(&mut s, len);

    while !is_valid(&s) {
        next(&mut s, len);
    }

    s
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(next_password(input))
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);

    input = next_password(input);

    Solution::from(next_password(input))
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()
        .remove(0)
}
