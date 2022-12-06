use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub type InputType = usize;

pub fn result_1(n: InputType) -> i64 {
    let mut scores: Vec<u32> = Vec::from([3, 7]);
    let mut elf0 = 0;
    let mut elf1 = 1;

    while scores.len() < n + 10 {
        scores.append(
            &mut (scores[elf0] + scores[elf1])
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect(),
        );

        elf0 = (elf0 + 1 + scores[elf0] as usize) % scores.len();
        elf1 = (elf1 + 1 + scores[elf1] as usize) % scores.len();
    }

    (n..n + 10).fold(0, |acc, index| acc * 10 + scores[index] as i64)
}

pub fn result_2(input: InputType) -> i64 {
    let digits: Vec<u32> = input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut scores = vec![HashSet::new(); 10];
    scores[3].insert(0);
    scores[7].insert(1);

    let mut current_serie = VecDeque::new();

    let mut elf0_index = 0;
    let mut elf1_index = 1;
    let mut elf0_score = 3;
    let mut elf1_score = 7;
    let mut els_count = 2;

    loop {
        for i in (elf0_score + elf1_score)
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
        {
            scores[i as usize].insert(els_count);
            els_count += 1;

            current_serie.push_back(i);
            if current_serie.len() > digits.len() {
                current_serie.pop_front();
            }
            if current_serie.eq(&digits) {
                return (els_count - digits.len()) as i64;
            }
        }

        elf0_index = (elf0_index + 1 + elf0_score) % els_count;
        elf1_index = (elf1_index + 1 + elf1_score) % els_count;

        for i in 0..10 {
            if scores[i].contains(&elf0_index) {
                elf0_score = i;
            }
            if scores[i].contains(&elf1_index) {
                elf1_score = i;
            }
        }
    }
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input[0].parse().unwrap()
}
