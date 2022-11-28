use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub type InputType = (VecDeque<usize>, VecDeque<usize>);

fn score(p: &VecDeque<usize>) -> usize {
    p.iter().rev().enumerate().map(|(i, x)| (i + 1) * x).sum()
}

pub fn result_1((mut p0, mut p1): InputType) -> i64 {
    while p0.len() > 0 && p1.len() > 0 {
        let c0 = p0.pop_front().unwrap();
        let c1 = p1.pop_front().unwrap();

        if c0 > c1 {
            p0.push_back(c0);
            p0.push_back(c1);
        } else {
            p1.push_back(c1);
            p1.push_back(c0);
        }
    }

    if p0.len() == 0 {
        score(&p1) as i64
    } else {
        score(&p0) as i64
    }
}

// winner id, score
fn rec_game(
    mut p0: VecDeque<usize>,
    mut p1: VecDeque<usize>,
    is_main_game: bool,
) -> (usize, usize) {
    if !is_main_game {
        let max0 = *p0.iter().max().unwrap();
        let max1 = *p1.iter().max().unwrap();

        if max1 > max0 && max1 > p0.len() + p1.len() - 2 {
            return (1, 2);
        }
    }

    let mut seen0: HashSet<VecDeque<usize>> = HashSet::new();
    let mut seen1: HashSet<VecDeque<usize>> = HashSet::new();

    while !seen0.contains(&p0) && !seen1.contains(&p1) {
        seen0.insert(p0.clone());
        seen1.insert(p1.clone());

        if p0.len() == 0 {
            return (1, if is_main_game { score(&p1) } else { 2 });
        } else if p1.len() == 0 {
            return (0, if is_main_game { score(&p0) } else { 2 });
        }

        let c0 = p0.pop_front().unwrap();
        let c1 = p1.pop_front().unwrap();

        let winner;

        if c0 <= p0.len() && c1 <= p1.len() {
            (winner, _) = rec_game(p0.clone(), p1.clone(), false);
        } else {
            winner = if c0 > c1 { 0 } else { 1 };
        }

        match winner {
            0 => {
                p0.push_back(c0);
                p0.push_back(c1);
            }
            1 => {
                p1.push_back(c1);
                p1.push_back(c0);
            }
            _ => panic!("unknown winner"),
        }
    }

    (1, if is_main_game { score(&p1) } else { 2 })
}

pub fn result_2((p0, p1): InputType) -> i64 {
    rec_game(p0, p1, true).1 as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let mut player0: VecDeque<usize> = VecDeque::new();
    let mut player1: VecDeque<usize> = VecDeque::new();
    let mut current_id = 0;

    for i in 1..input.len() {
        let line: &str = &input[i];
        if let Ok(x) = line.parse::<usize>() {
            match current_id {
                0 => player0.push_back(x),
                1 => player1.push_back(x),
                _ => panic!("wrong id"),
            }
        } else {
            current_id += 1;
        }
    }

    (player0, player1)
}
