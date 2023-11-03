use crate::Solution;

type InputType = Vec<String>;

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut count: i64 = 0;
    let mut begin_index = None;

    for l in input {
        let chars = l.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            if chars[i].is_digit(10) || chars[i] == '-' {
                if begin_index.is_none() {
                    begin_index = Some(i);
                }
            } else {
                if let Some(b) = begin_index {
                    count += l[b..i].parse::<i64>().unwrap();
                    begin_index = None;
                }
            }
        }

        if let Some(b) = begin_index {
            count += l[b..].parse::<i64>().unwrap();
        }

        begin_index = None;
    }

    Solution::from(count)
}

pub fn contains_red_as_arg(s: &str) -> bool {
    let mut brackets_count = 0;
    let mut arrays_count = 0;
    let mut is_right_item = false;
    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..(chars.len() - 2) {
        match chars[i] {
            '{' => brackets_count += 1,
            '}' => brackets_count -= 1,
            '[' => arrays_count += 1,
            ']' => arrays_count -= 1,
            ',' => {
                if arrays_count == 0 && brackets_count == 1 {
                    is_right_item = false;
                }
            }
            ':' => {
                if arrays_count == 0 && brackets_count == 1 {
                    is_right_item = true;
                }
            }
            _ => (),
        }

        if is_right_item && arrays_count == 0 && brackets_count == 1 {
            if chars[i] == 'r' && chars[i + 1] == 'e' && chars[i + 2] == 'd' {
                return true;
            }
        }
    }

    false
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut count: i64 = 0;
    let mut begin_index = None;
    let mut begin_brackets_indexs: Vec<usize> = vec![];
    let mut temp_counts: Vec<i64> = vec![];

    for l in input {
        let chars = l.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            if chars[i].is_digit(10) || chars[i] == '-' {
                if begin_index.is_none() {
                    begin_index = Some(i);
                }
            } else {
                if let Some(b) = begin_index {
                    if !begin_brackets_indexs.is_empty() {
                        *temp_counts.last_mut().unwrap() += l[b..i].parse::<i64>().unwrap();
                    } else {
                        count += l[b..i].parse::<i64>().unwrap();
                    }
                    begin_index = None;
                }
            }

            if chars[i] == '{' {
                begin_brackets_indexs.push(i);
                temp_counts.push(0);
            } else if chars[i] == '}' {
                let b = begin_brackets_indexs.pop().unwrap();
                let tc = temp_counts.pop().unwrap();
                if !contains_red_as_arg(&l[b..i]) {
                    if let Some(last) = temp_counts.last_mut() {
                        *last += tc;
                    } else {
                        count += tc;
                    }
                }
            }
        }

        if let Some(b) = begin_index {
            count += l[b..].parse::<i64>().unwrap();
        }

        begin_index = None;
        begin_brackets_indexs.clear();
        temp_counts.clear();
    }

    Solution::from(count)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect()
}
