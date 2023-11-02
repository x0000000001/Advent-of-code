use std::collections::HashMap;

use crate::Solution;

type InputType = (Vec<char>, HashMap<(char, char), char>);

pub fn part1(s: String) -> Solution {
    let (mut molecule, rules) = parse(s);

    for _ in 0..10 {
        let mut index = 0;
        for _ in 0..molecule.len() - 1 {
            molecule.insert(
                index + 1,
                *rules.get(&(molecule[index], molecule[index + 1])).unwrap(),
            );

            index += 2;
        }
    }

    let mut counts: HashMap<char, i64> = HashMap::new();

    for c in molecule {
        if counts.contains_key(&c) {
            counts.insert(c, counts.get(&c).unwrap() + 1);
        } else {
            counts.insert(c, 1);
        }
    }

    let mut min = i64::MAX;
    let mut max = 0;

    for v in counts.values() {
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }

    Solution::from(max - min)
}

pub fn part2(s: String) -> Solution {
    let (mut molecule_temp, rules) = parse(s);

    //representing molecule with dic, where key is number of times a pair is present in it

    let mut molecule: HashMap<(char, char), i64> = HashMap::new();
    //init
    for i in 0..molecule_temp.len() - 1 {
        let pair = (molecule_temp[i], molecule_temp[i + 1]);

        let counter = molecule.entry(pair).or_insert(0);
        *counter += 1;
    }

    //during count, 1st and last elements won't be counted twice as opposed to the others
    //since they remain the same, counting them here
    let first_el = molecule_temp[0];
    let last_el = molecule_temp.pop().unwrap();

    for _ in 0..40 {
        let mut new_pairs: HashMap<(char, char), i64> = HashMap::new();
        for (pair, val) in molecule {
            let inserted_char = rules.get(&pair).unwrap().clone();
            let left_pair = (pair.0, inserted_char);
            let rigth_pair = (inserted_char, pair.1);

            let counter = new_pairs.entry(left_pair).or_insert(0);
            *counter += val;

            let counter = new_pairs.entry(rigth_pair).or_insert(0);
            *counter += val;
        }

        molecule = new_pairs;
    }

    let mut counts: HashMap<char, i64> = HashMap::new();

    for (pair, val) in molecule {
        let counter = counts.entry(pair.0).or_insert(0);
        *counter += val;
        let counter = counts.entry(pair.1).or_insert(0);
        *counter += val;
    }

    counts.insert(first_el, counts.get(&first_el).unwrap() + 1);
    counts.insert(last_el, counts.get(&last_el).unwrap() + 1);

    let mut min = i64::MAX;
    let mut max = 0;

    for v in counts.values() {
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }

    Solution::from((max - min) / 2)
}

fn parse(s: String) -> InputType {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let start: Vec<char> = input[0].chars().collect();

    let mut i = 2;
    let len = input.len();

    let mut rules: HashMap<(char, char), char> = HashMap::new();

    while i < len {
        let words: Vec<Vec<char>> = input[i]
            .split(" -> ")
            .map(|el| el.chars().collect())
            .collect();

        rules.insert((words[0][0], words[0][1]), words[1][0]);

        i += 1;
    }

    (start, rules)
}
