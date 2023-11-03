use std::collections::HashSet;

use crate::Solution;

type InputType = Vec<Position>;

pub type Position = (usize, usize, usize);

fn are_neighbors(p0: &Position, p1: &Position) -> bool {
    (p0.0.abs_diff(p1.0) + p0.1.abs_diff(p1.1) + p0.2.abs_diff(p1.2)) < 2
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let n = input.len();
    Solution::from(
        (n * 6
            - 2 * (0..n)
                .map(|i| {
                    ((i + 1)..n)
                        .map(|j| {
                            if are_neighbors(&input[i], &input[j]) {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()) as i64,
    )
}

fn neighbors((x, y, z): Position, maxx: usize, maxy: usize, maxz: usize) -> Vec<Position> {
    let mut neighbors = vec![];

    if x > 0 {
        neighbors.push((x - 1, y, z));
    }
    if x < maxx {
        neighbors.push((x + 1, y, z));
    }
    if y > 0 {
        neighbors.push((x, y - 1, z));
    }
    if y < maxy {
        neighbors.push((x, y + 1, z));
    }
    if z > 0 {
        neighbors.push((x, y, z - 1));
    }
    if z < maxz {
        neighbors.push((x, y, z + 1));
    }

    neighbors
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let n = input.len();

    let mut count = n * 6
        - 2 * (0..n)
            .map(|i| {
                ((i + 1)..n)
                    .map(|j| {
                        if are_neighbors(&input[i], &input[j]) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

    let maxx = *input.iter().map(|(x, _, _)| x).max().unwrap();
    let maxy = *input.iter().map(|(_, y, _)| y).max().unwrap();
    let maxz = *input.iter().map(|(_, _, z)| z).max().unwrap();
    let lava = input.clone().into_iter().collect::<HashSet<Position>>();

    let mut outside_class = HashSet::new();
    let mut queue = vec![];
    queue.push((0, 0, 0));
    queue.push((maxx, 0, 0));
    queue.push((maxx, maxy, 0));
    queue.push((maxx, 0, maxz));
    queue.push((0, maxy, 0));
    queue.push((0, maxy, maxz));
    queue.push((0, 0, maxz));
    queue.push((maxx, maxy, maxz));

    while !queue.is_empty() {
        let p = queue.pop().unwrap();

        for pc in neighbors(p, maxx, maxy, maxz) {
            if !lava.contains(&pc) && !outside_class.contains(&pc) {
                outside_class.insert(pc);
                queue.push(pc);
            }
        }
    }

    for x in 0..maxx {
        for y in 0..maxy {
            for z in 0..maxz {
                let p = (x, y, z);
                if !lava.contains(&p) && !outside_class.contains(&p) {
                    for pc in neighbors(p, maxx, maxy, maxz) {
                        if lava.contains(&pc) {
                            count -= 1;
                        }
                    }
                }
            }
        }
    }

    Solution::from(count as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input
        .into_iter()
        .map(|l| {
            l.split(',')
                .map(|w| w.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|ws| (ws[0], ws[1], ws[2]))
        .collect()
}
