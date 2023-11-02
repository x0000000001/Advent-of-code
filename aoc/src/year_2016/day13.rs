use std::collections::HashMap;

use crate::Solution;

type InputType = usize;

fn is_free((x, y): (usize, usize), param: usize) -> bool {
    let mut res = (x * x + 3 * x + 2 * x * y + y + y * y) + param;
    let mut pair_bits_count = true;
    while res != 0 {
        if res % 2 == 0 {
            pair_bits_count = !pair_bits_count;
            res -= 1;
        }

        res /= 2;
    }

    pair_bits_count
}

/// Djisktra, one again.
pub fn path_len(start: (usize, usize), end: (usize, usize), param: usize) -> u64 {
    // cases which possibility of going is known (since maze cases have to be processed)
    let mut known_cases: HashMap<(usize, usize), bool> = HashMap::new();
    known_cases.insert(start, true);

    let mut explored_cases: Vec<(usize, usize)> = vec![];
    let mut queue: Vec<((usize, usize), u64)> = vec![(start, 0); 1];

    while !queue.is_empty() {
        queue.sort_by(|(_, score0), (_, score1)| score1.cmp(score0));

        let (pos, score) = queue.pop().unwrap();

        if pos == end {
            return score;
        }

        let mut candidate_pos = Vec::from([(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)]);
        if pos.0 > 0 {
            candidate_pos.push((pos.0 - 1, pos.1))
        };
        if pos.1 > 0 {
            candidate_pos.push((pos.0, pos.1 - 1))
        };

        'candidate_loop: for new_pos in candidate_pos {
            if new_pos == pos {
                continue;
            }

            let is_free = known_cases
                .entry(new_pos)
                .or_insert(is_free(new_pos, param));

            if !*is_free {
                continue;
            }

            if explored_cases.contains(&new_pos) {
                continue;
            }

            for i in 0..queue.len() {
                if queue[i].0 == new_pos {
                    queue[i].1 = queue[i].1.min(score + 1);
                    continue 'candidate_loop;
                }
            }

            queue.push((new_pos, score + 1));
        }

        explored_cases.push(pos);
    }

    0
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(path_len((1, 1), (31, 39), input) as i64)
}

/// Djisktra, adapted to counting locations accessible within a certain range
pub fn accessible_cases(start: (usize, usize), param: usize, max_range: u64) -> u64 {
    // cases which possibility of going is known (since maze cases have to be processed)
    let mut known_cases: HashMap<(usize, usize), bool> = HashMap::new();
    known_cases.insert(start, true);

    let mut explored_cases: Vec<(usize, usize)> = vec![];
    let mut queue: Vec<((usize, usize), u64)> = vec![(start, 0); 1];

    while !queue.is_empty() {
        queue.sort_by(|(_, score0), (_, score1)| score1.cmp(score0));

        let (pos, score) = queue.pop().unwrap();

        if score == max_range {
            explored_cases.push(pos);
            continue;
        }

        let mut candidate_pos = Vec::from([(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)]);
        if pos.0 > 0 {
            candidate_pos.push((pos.0 - 1, pos.1))
        };
        if pos.1 > 0 {
            candidate_pos.push((pos.0, pos.1 - 1))
        };

        'candidate_loop: for new_pos in candidate_pos {
            if new_pos == pos {
                continue;
            }

            let is_free = known_cases
                .entry(new_pos)
                .or_insert(is_free(new_pos, param));

            if !*is_free {
                continue;
            }

            if explored_cases.contains(&new_pos) {
                continue;
            }

            for i in 0..queue.len() {
                if queue[i].0 == new_pos {
                    queue[i].1 = queue[i].1.min(score + 1);
                    continue 'candidate_loop;
                }
            }

            queue.push((new_pos, score + 1));
        }

        explored_cases.push(pos);
    }

    explored_cases.len() as u64
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(accessible_cases((1, 1), input, 50) as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .parse()
        .unwrap()
}
