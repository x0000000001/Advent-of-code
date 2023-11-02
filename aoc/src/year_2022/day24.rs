use crate::Solution;
use std::collections::{HashMap, HashSet, VecDeque};

// height, width at the end
type InputType = (Blizzards, usize, usize);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    Est,
    South,
    West,
}

pub type Blizzards = HashMap<Position, Vec<Direction>>;

#[allow(dead_code)]
fn print_blizzard(blizzards: &Blizzards, h: usize, w: usize, player: Option<Position>) {
    for x in 0..h {
        for y in 0..w {
            print!(
                "{}",
                if player.is_some() && player.unwrap().eq(&Position { x, y }) {
                    'E'
                } else if x == 0 || y == 0 || x == h - 1 || y == w - 1 {
                    '#'
                } else {
                    if let Some(dirs) = blizzards.get(&Position { x, y }) {
                        if dirs.len() > 1 {
                            '*'
                        } else {
                            match dirs[0] {
                                Direction::North => '^',
                                Direction::Est => '>',
                                Direction::South => 'v',
                                Direction::West => '<',
                            }
                        }
                    } else {
                        '.'
                    }
                }
            );
        }

        println!();
    }
    println!();
}

fn iter_blizzards(blizzards: &Blizzards, h: usize, w: usize) -> Blizzards {
    let mut new_b = HashMap::new();

    for (pos, dirs) in blizzards {
        for dir in dirs {
            let new_pos = match dir {
                Direction::North => Position {
                    x: if pos.x == 1 { h - 2 } else { pos.x - 1 },
                    y: pos.y,
                },
                Direction::Est => Position {
                    x: pos.x,
                    y: if pos.y == w - 2 { 1 } else { pos.y + 1 },
                },
                Direction::South => Position {
                    x: if pos.x == h - 2 { 1 } else { pos.x + 1 },
                    y: pos.y,
                },
                Direction::West => Position {
                    x: pos.x,
                    y: if pos.y == 1 { w - 2 } else { pos.y - 1 },
                },
            };

            let entry = new_b.entry(new_pos).or_insert(vec![]);
            entry.push(*dir);
        }
    }

    new_b
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    if x > y {
        (x, y) = (y, x);
    }

    if y % x == 0 {
        x
    } else {
        gcd(y % x, x)
    }
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

pub fn part1(s: String) -> Solution {
    let (blizzards, h, w) = parse(s);

    let start = Position { x: 0, y: 1 };
    let end = Position { x: h - 2, y: w - 2 };
    let forcast_count = lcm(h - 2, w - 2);
    let mut states = vec![];
    states.push(blizzards);
    for _ in 1..forcast_count {
        // print_blizzard(states.last().unwrap(), h, w, None);
        states.push(iter_blizzards(states.last().unwrap(), h, w));
    }

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_front((start, 0));
    seen.insert((start, 0));

    while let Some((pos, t)) = queue.pop_back() {
        // print_blizzard(&states[t % forcast_count], h, w, Some(pos));

        if pos.eq(&end) {
            return Solution::from(t as i64 + 1);
        }

        let state = &states[(t + 1) % forcast_count];
        let mut neighbors = vec![];
        let new_t = t + 1;

        // SOUTH
        if pos.x < h - 2 {
            let est_pos = Position {
                x: pos.x + 1,
                y: pos.y,
            };
            if !state.contains_key(&est_pos) {
                neighbors.push((est_pos, new_t));
            }
        }

        // player was moving in the walls at the start
        if !pos.eq(&start) {
            // NORTH
            if pos.x > 1 {
                let west_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !state.contains_key(&west_pos) {
                    neighbors.push((west_pos, new_t));
                }
            }
            // WEST
            if pos.y > 1 {
                let north_pos = Position {
                    x: pos.x,
                    y: pos.y - 1,
                };
                if !state.contains_key(&north_pos) {
                    neighbors.push((north_pos, new_t));
                }
            }
            // EST
            if pos.y < w - 2 {
                let south_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !state.contains_key(&south_pos) {
                    neighbors.push((south_pos, new_t));
                }
            }
        }

        if !state.contains_key(&pos) {
            neighbors.push((pos, new_t));
        }

        for n in neighbors {
            if !seen.contains(&n) {
                seen.insert(n);
                queue.push_front(n);
            }
        }
    }

    Solution::NotFound
}

pub fn part2(s: String) -> Solution {
    let (blizzards, h, w) = parse(s);

    let start = Position { x: 0, y: 1 };
    let mut goal = Position { x: h - 2, y: w - 2 };
    let forcast_count = lcm(h - 2, w - 2);
    let mut states = vec![];
    states.push(blizzards);
    for _ in 1..forcast_count {
        // print_blizzard(states.last().unwrap(), h, w, None);
        states.push(iter_blizzards(states.last().unwrap(), h, w));
    }

    let mut current_step = 0;

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_front((start, 0));
    seen.insert((start, 0));

    while let Some((pos, t)) = queue.pop_back() {
        // print_blizzard(&states[t % forcast_count], h, w, Some(pos));

        if pos.eq(&goal) {
            match current_step {
                0 => {
                    seen.clear();
                    queue.clear();
                    queue.push_front((Position { x: h - 1, y: w - 2 }, t + 1));
                    goal = Position { x: 1, y: 1 }
                }
                1 => {
                    seen.clear();
                    queue.clear();
                    queue.push_front((Position { x: 0, y: 1 }, t + 1));
                    goal = Position { x: h - 2, y: w - 2 };
                }
                2 => return Solution::from(t as i64 + 1),
                _ => panic!(),
            }

            current_step += 1;
            continue;
        }

        let state = &states[(t + 1) % forcast_count];
        let mut neighbors = vec![];
        let new_t = t + 1;

        // player was moving in the walls at the start
        if pos.x == 0 && pos.y == 1 {
            // SOUTH
            if pos.x < h - 2 {
                let est_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !state.contains_key(&est_pos) {
                    neighbors.push((est_pos, new_t));
                }
            }
        } else if pos.x == h - 1 && pos.y == w - 2 {
            // NORTH
            if pos.x > 1 {
                let west_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !state.contains_key(&west_pos) {
                    neighbors.push((west_pos, new_t));
                }
            }
        } else {
            // NORTH
            if pos.x > 1 {
                let west_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !state.contains_key(&west_pos) {
                    neighbors.push((west_pos, new_t));
                }
            }
            // SOUTH
            if pos.x < h - 2 {
                let est_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !state.contains_key(&est_pos) {
                    neighbors.push((est_pos, new_t));
                }
            }
            // WEST
            if pos.y > 1 {
                let north_pos = Position {
                    x: pos.x,
                    y: pos.y - 1,
                };
                if !state.contains_key(&north_pos) {
                    neighbors.push((north_pos, new_t));
                }
            }
            // EST
            if pos.y < w - 2 {
                let south_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !state.contains_key(&south_pos) {
                    neighbors.push((south_pos, new_t));
                }
            }
        }

        if !state.contains_key(&pos) {
            neighbors.push((pos, new_t));
        }

        for n in neighbors {
            if !seen.contains(&n) {
                seen.insert(n);
                queue.push_front(n);
            }
        }
    }

    Solution::NotFound
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let mut res = HashMap::new();

    for x in 1..(input.len() - 1) {
        let mut chars = input[x].chars();
        chars.next();
        for y in 1..(input[0].len() - 1) {
            let c = chars.next().unwrap();
            if c == '.' {
                continue;
            }

            let dir = match c {
                '^' => Direction::North,
                '>' => Direction::Est,
                'v' => Direction::South,
                '<' => Direction::West,
                _ => panic!(),
            };

            res.insert(Position { x, y }, Vec::from([dir]));
        }
    }

    (res, input.len(), input[0].len())
}
