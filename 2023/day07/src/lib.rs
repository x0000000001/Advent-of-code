mod helpers;

use std::{cmp, collections::HashMap};

pub use helpers::Solution;

type InputType = Vec<(Hand, usize)>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPairs,
    OnePair,
    High,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    handtype: Option<HandType>,
}

fn cmp(hand0: &Hand, hand1: &Hand, char_score: &dyn Fn(char) -> usize) -> cmp::Ordering {
    match hand0.handtype.cmp(&hand1.handtype) {
        core::cmp::Ordering::Equal => {
            for i in 0..5 {
                let score0 = char_score(hand0.cards[i]);
                let score1 = char_score(hand1.cards[i]);

                match score0.cmp(&score1) {
                    cmp::Ordering::Equal => continue,
                    ord => return ord,
                }
            }

            cmp::Ordering::Equal
        }
        ord => ord,
    }
}

impl Hand {
    fn get_type(occs: &HashMap<char, usize>) -> HandType {
        let mut occs_list = occs.values().copied().collect::<Vec<usize>>();
        occs_list.sort();

        match occs_list.len() {
            1 => HandType::Five,
            2 => {
                if occs_list[0] == 1 {
                    HandType::Four
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if occs_list[1] == 1 {
                    HandType::Three
                } else {
                    HandType::TwoPairs
                }
            }
            4 => HandType::OnePair,
            5 => HandType::High,
            _ => panic!(),
        }
    }

    fn construct_type0(&mut self) {
        let mut occs = HashMap::new();

        for &c in self.cards.iter() {
            let entry = occs.entry(c).or_insert(0);
            *entry += 1;
        }

        self.handtype = Some(Hand::get_type(&occs));
    }

    fn construct_type1(&mut self) {
        let mut occs = HashMap::new();

        for &c in self.cards.iter() {
            let entry = occs.entry(c).or_insert(0);
            *entry += 1;
        }

        let jokers_count = occs.remove(&'J').unwrap_or(0);

        if jokers_count == 5 {
            self.handtype = Some(HandType::Five);
            return;
        }

        let max_key = *occs.iter().max_by_key(|&(_, count)| count).unwrap().0;
        *occs.entry(max_key).or_insert(0) += jokers_count;

        self.handtype = Some(Hand::get_type(&occs));
    }
}

fn char_score0(c: char) -> usize {
    match c {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 4,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
        '5' => 10,
        '4' => 11,
        '3' => 12,
        '2' => 13,
        _ => panic!(),
    }
}

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    for (hand, _) in input.iter_mut() {
        hand.construct_type0();
    }

    input.sort_by(|(hand0, _), (hand1, _)| cmp(hand1, hand0, &char_score0));

    Solution::from(
        input
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| bid * (i + 1))
            .sum::<usize>() as u64,
    )
}

fn char_score1(c: char) -> usize {
    match c {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
        '5' => 10,
        '4' => 11,
        '3' => 12,
        '2' => 13,
        'J' => 14,
        _ => panic!(),
    }
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);

    for (hand, _) in input.iter_mut() {
        hand.construct_type1();
    }

    input.sort_by(|(hand0, _), (hand1, _)| cmp(hand1, hand0, &char_score1));

    Solution::from(
        input
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| bid * (i + 1))
            .sum::<usize>() as u64,
    )
}

fn parse(s: String) -> InputType {
    s.lines()
        .map(|l| {
            let w = l.split_whitespace().collect::<Vec<&str>>();

            (
                Hand {
                    cards: w[0].chars().collect(),
                    handtype: None,
                },
                w[1].parse().unwrap(),
            )
        })
        .collect()
}
