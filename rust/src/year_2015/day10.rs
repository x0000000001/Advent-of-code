use crate::Solution;

type InputType = Vec<u32>;

pub fn step(input: &mut InputType) {
    let mut temp: InputType = vec![];

    let mut current_num = input[0];
    let mut current_count = 0;

    for j in 0..input.len() {
        if input[j] == current_num {
            current_count += 1;
        } else {
            temp.push(current_count);
            temp.push(current_num);
            current_count = 1;
            current_num = input[j];
        }
    }

    temp.push(current_count);
    temp.push(current_num);

    input.clear();
    input.append(&mut temp);
}

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);
    for _ in 0..40 {
        step(&mut input);
    }

    Solution::from(input.len() as i64)
}

/// hmm... well the naïve way works as well for problem 2... \
/// in < 2s... \
/// so hhmm... I guess that's done ?
pub fn part2(s: String) -> Solution {
    let mut input = parse(s);
    for _ in 0..50 {
        step(&mut input);
    }

    Solution::from(input.len() as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
