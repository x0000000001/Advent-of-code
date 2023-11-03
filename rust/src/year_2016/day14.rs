use std::collections::VecDeque;

use crate::Solution;

type InputType = String;

fn is_hash_valid(h: &str, hashes: &VecDeque<String>) -> bool {
    let mut curent_c_count = 0;
    let mut current_c = ' ';

    for c in h.chars() {
        if c == current_c {
            curent_c_count += 1;
        } else {
            curent_c_count = 1;
            current_c = c;
        }

        if curent_c_count == 3 {
            for h1 in hashes {
                let mut current_c_count_1 = 0;
                for c1 in h1.chars() {
                    if c1 == current_c {
                        current_c_count_1 += 1;
                        if current_c_count_1 == 5 {
                            return true;
                        }
                    } else {
                        current_c_count_1 = 0;
                    }
                }
            }

            return false;
        }
    }

    false
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut valid_hash_count = 0;
    let mut index = 0;
    let mut hashes: VecDeque<String> = VecDeque::with_capacity(1000);

    let get_hash = |i: usize| -> String {
        let digest = md5::compute(format!("{input}{i}"));
        format!("{:x}", digest)
    };

    for i in index..(index + 1000) {
        hashes.push_back(get_hash(i));
    }

    while valid_hash_count < 64 {
        let h = hashes.pop_front().unwrap();
        hashes.push_back(get_hash(index + 1000));

        if is_hash_valid(&h, &hashes) {
            valid_hash_count += 1;
        }

        index += 1;
    }

    Solution::from((index - 1) as i64)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut valid_hash_count = 0;
    let mut index = 0;
    let mut hashes: VecDeque<String> = VecDeque::with_capacity(1000);

    let get_hash = |i: usize| -> String {
        let mut digest = format!("{input}{i}");
        for _ in 0..2017 {
            digest = format!("{:x}", md5::compute(digest));
        }
        digest
    };

    for i in index..(index + 1000) {
        hashes.push_back(get_hash(i));
    }

    while valid_hash_count < 64 {
        let h = hashes.pop_front().unwrap();
        hashes.push_back(get_hash(index + 1000));

        if is_hash_valid(&h, &hashes) {
            valid_hash_count += 1;
        }

        index += 1;
    }

    Solution::from((index - 1) as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .clone()
}
