use std::collections::{HashMap, HashSet};

use crate::Solution;

type InputType = (HashSet<(i64, i64)>, usize);

pub fn part1(s: String) -> Solution {
    let (mut s, n) = parse(s);

    let (mut x, mut y, mut dir) = ((n + 1) as i64 / 2 - 1, (n + 1) as i64 / 2 - 1, 0);
    let mut count = 0;

    for _ in 0..10000 {
        let infected = s.contains(&(x, y));
        if infected {
            s.remove(&(x, y));
            dir = (dir + 1) % 4;
        } else {
            count += 1;
            s.insert((x, y).clone());
            dir = (dir + 3) % 4;
        }

        match dir {
            0 => x -= 1,
            1 => y += 1,
            2 => x += 1,
            3 => y -= 1,
            _ => panic!(),
        }
    }

    Solution::from(count)
}

enum State {
    Infected,
    Weakened,
    Flagged,
}

pub fn part2(s: String) -> Solution {
    let (s, n) = parse(s);

    let (mut x, mut y, mut dir) = ((n + 1) as i64 / 2 - 1, (n + 1) as i64 / 2 - 1, 0);

    let mut m: HashMap<(i64, i64), State> = HashMap::new();

    for k in s.into_iter() {
        m.insert(k, State::Infected);
    }

    let mut count = 0;

    for _ in 0..10000000 {
        let is_clean = !m.contains_key(&(x, y));

        if is_clean {
            m.insert((x, y).clone(), State::Weakened);
            dir = (dir + 3) % 4;
        } else {
            match m.get(&(x, y)).unwrap() {
                State::Flagged => {
                    m.remove(&(x, y));
                    dir = (dir + 2) % 4;
                }
                State::Infected => {
                    m.insert((x, y).clone(), State::Flagged);
                    dir = (dir + 1) % 4;
                }
                State::Weakened => {
                    count += 1;
                    m.insert((x, y).clone(), State::Infected);
                    ()
                }
            }
        }

        match dir {
            0 => x -= 1,
            1 => y += 1,
            2 => x += 1,
            3 => y -= 1,
            _ => panic!(),
        }
    }

    Solution::from(count)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let n = input.len();
    let mut s = HashSet::new();

    for i in 0..input.len() {
        let chars = input[i].chars().collect::<Vec<char>>();
        for j in 0..chars.len() {
            if chars[j] == '#' {
                s.insert((i as i64, j as i64));
            }
        }
    }

    (s, n)
}
