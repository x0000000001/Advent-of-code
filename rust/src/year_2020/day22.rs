use crate::Solution;
use std::collections::{HashSet, VecDeque};

type InputType = (VecDeque<usize>, VecDeque<usize>);

fn score(p: &VecDeque<usize>) -> usize {
    p.iter().rev().enumerate().map(|(i, x)| (i + 1) * x).sum()
}

pub fn part1(s: String) -> Solution {
    let (mut p0, mut p1) = parse(s);

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
        Solution::from(score(&p1) as i64)
    } else {
        Solution::from(score(&p0) as i64)
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

        if max0 > max1 && max0 > p0.len() + p1.len() {
            return (0, 0);
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
            let new_deck0 = p0.range(0..c0).copied().collect();
            let new_deck1 = p1.range(0..c1).copied().collect();

            let res = rec_game(new_deck0, new_deck1, false);
            winner = res.0;
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

    (0, if is_main_game { score(&p0) } else { 0 })
}

pub fn part2(s: String) -> Solution {
    let (p0, p1) = parse(s);

    Solution::from(rec_game(p0, p1, true).1 as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
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
