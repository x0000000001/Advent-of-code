use std::{cmp::Ordering, collections::HashMap};

use crate::Solution;

type InputType = Vec<(Vec<String>, Vec<char>, i64)>;

pub fn is_valid((names, letters, _): &(Vec<String>, Vec<char>, i64)) -> bool {
    let mut counts: HashMap<char, i64> = HashMap::new();

    for n in names {
        for c in n.chars() {
            let accessor = counts.entry(c).or_insert(0);
            *accessor += 1;
        }
    }

    let mut temp = counts.into_iter().collect::<Vec<(char, i64)>>();
    temp.sort_by(|(char0, c0), (char1, c1)| {
        let res = c1.cmp(c0);
        if res == Ordering::Equal {
            char0.cmp(char1)
        } else {
            res
        }
    });
    let most_recurrent_letters = temp.into_iter().map(|(c, _)| c).collect::<Vec<char>>();
    most_recurrent_letters[0..5] == *letters
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(
        input
            .into_iter()
            .filter(|el| is_valid(&el))
            .map(|(_, _, c)| c)
            .sum::<i64>(),
    )
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for (words, _, n) in input.into_iter().filter(|el| is_valid(el)) {
        let chars: Vec<char> = words
            .into_iter()
            .flat_map(|el| el.chars().collect::<Vec<char>>())
            .collect();

        let indices: Vec<usize> = chars
            .iter()
            .map(|el| alphabet.iter().position(|l| l == el).unwrap())
            .collect();
        let indices: Vec<usize> = indices.into_iter().map(|p| (p + n as usize) % 26).collect();

        let new_chars: String = indices.into_iter().map(|el| alphabet[el]).collect();
        if new_chars.contains("northpole") {
            return Solution::from(n);
        }
    }

    Solution::from(0)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for l in input {
        let w = l.split("-").collect::<Vec<&str>>();
        let w1 = w.last().unwrap().split("[").collect::<Vec<&str>>();

        res.push((
            w[0..(w.len() - 1)]
                .into_iter()
                .map(|el| el.to_string())
                .collect(),
            w1[1][0..(w1[1].len() - 1)].to_string().chars().collect(),
            w1[0].parse::<i64>().unwrap(),
        ));
    }

    res
}
