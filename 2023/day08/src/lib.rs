mod helpers;

use std::collections::HashMap;

pub use helpers::Solution;

type Pos = (char, char, char);
type InputType = (Vec<char>, Vec<(Pos, Pos, Pos)>);

fn node_length(
    start: Pos,
    instrs: &Vec<char>,
    dirs: &HashMap<Pos, (Pos, Pos)>,
    end_easy: bool,
) -> usize {
    let mut current_pos = start;
    let mut current_instr_index = 0;
    let mut count = 0;

    while (!end_easy && current_pos != ('Z', 'Z', 'Z')) || (end_easy && current_pos.2 != 'Z') {
        let entry = dirs.get(&current_pos).unwrap();
        current_pos = match instrs[current_instr_index] {
            'R' => entry.1,
            'L' => entry.0,
            _ => panic!(),
        };

        current_instr_index = (current_instr_index + 1) % instrs.len();
        count += 1;
    }

    count
}

pub fn part1(s: String) -> Solution {
    let (instrs, lines) = parse(s);

    let dirs: HashMap<Pos, (Pos, Pos)> =
        HashMap::from_iter(lines.into_iter().map(|(k, l, r)| (k, (l, r))));

    let start = ('A', 'A', 'A');

    Solution::from(node_length(start, &instrs, &dirs, false) as u64)
}

fn pgcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        (a, b) = (b, a);
    }

    while b != 0 {
        let tmp = a % b;
        (a, b) = (b, tmp);
    }

    a
}

fn ppcm(a: usize, b: usize) -> usize {
    (a * b) / pgcd(a, b)
}

// 1168818281 too low
pub fn part2(s: String) -> Solution {
    let (instrs, lines) = parse(s);

    let dirs: HashMap<Pos, (Pos, Pos)> =
        HashMap::from_iter(lines.into_iter().map(|(k, l, r)| (k, (l, r))));

    dbg!(dirs
        .keys()
        .filter(|&k| k.2 == 'A')
        .map(|&start| node_length(start, &instrs, &dirs, true))
        .collect::<Vec<usize>>());

    Solution::from(
        dirs.keys()
            .filter(|&k| k.2 == 'A')
            .map(|&start| node_length(start, &instrs, &dirs, true))
            .reduce(ppcm)
            .unwrap() as u64,
    )
}

fn parse(s: String) -> InputType {
    let mut lines = s.lines();

    let instrs = lines.next().unwrap().chars().collect();
    lines.next();

    (
        instrs,
        lines
            .filter_map(|l| {
                let chars = l.chars().collect::<Vec<char>>();
                if chars.is_empty() {
                    return None;
                }

                Some((
                    (chars[0], chars[1], chars[2]),
                    (chars[7], chars[8], chars[9]),
                    (chars[12], chars[13], chars[14]),
                ))
            })
            .collect(),
    )
}
