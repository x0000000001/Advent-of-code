use std::collections::{HashMap, HashSet, VecDeque};

use crate::Solution;

type InputType = HashSet<Position>;

pub type Position = (i64, i64);

fn iter_elves(elves: InputType, directions: &VecDeque<usize>) -> (HashSet<Position>, bool) {
    let mut new_s = HashSet::new();
    let mut have_moved = false;

    let mut propositions: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();

    for &(x, y) in elves.iter() {
        let neighbors: Vec<(i64, i64)> = (-1..2)
            .flat_map(move |i| (-1..2).map(move |j| (x + i, y + j)))
            .filter(|&(xc, yc)| !(xc == x && yc == y) && elves.contains(&(xc, yc)))
            .collect();

        if neighbors.is_empty() {
            new_s.insert((x, y));
            continue;
        }

        let mut proposition = None;

        for &d in directions.iter() {
            match d {
                0 => {
                    if !neighbors.contains(&(x - 1, y))
                        && !neighbors.contains(&(x - 1, y - 1))
                        && !neighbors.contains(&(x - 1, y + 1))
                    {
                        proposition = Some((x - 1, y));
                        break;
                    }
                }
                1 => {
                    if !neighbors.contains(&(x - 1, y + 1))
                        && !neighbors.contains(&(x, y + 1))
                        && !neighbors.contains(&(x + 1, y + 1))
                    {
                        proposition = Some((x, y + 1));
                        break;
                    }
                }
                2 => {
                    if !neighbors.contains(&(x + 1, y - 1))
                        && !neighbors.contains(&(x + 1, y))
                        && !neighbors.contains(&(x + 1, y + 1))
                    {
                        proposition = Some((x + 1, y));
                        break;
                    }
                }
                3 => {
                    if !neighbors.contains(&(x - 1, y - 1))
                        && !neighbors.contains(&(x, y - 1))
                        && !neighbors.contains(&(x + 1, y - 1))
                    {
                        proposition = Some((x, y - 1));
                        break;
                    }
                }
                _ => panic!(),
            }
        }

        if let Some(p) = proposition {
            let entry = propositions.entry(p).or_insert(vec![]);
            entry.push((x, y));
        } else {
            new_s.insert((x, y));
        }
    }

    for (position, candidates) in propositions {
        if candidates.len() > 1 {
            for c in candidates {
                new_s.insert(c);
            }
        } else {
            new_s.insert(position);
            have_moved = true;
        }
    }

    (new_s, have_moved)
}

#[allow(dead_code)]
fn print(elves: &InputType) {
    let minx = *elves.iter().map(|(x, _)| x).min().unwrap();
    let maxx = *elves.iter().map(|(x, _)| x).max().unwrap();
    let miny = *elves.iter().map(|(_, y)| y).min().unwrap();
    let maxy = *elves.iter().map(|(_, y)| y).max().unwrap();

    for i in minx..(maxx + 1) {
        for j in miny..(maxy + 1) {
            print!("{}", if elves.contains(&(i, j)) { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

fn score(elves: &InputType) -> i64 {
    let minx = *elves.iter().map(|(x, _)| x).min().unwrap();
    let maxx = *elves.iter().map(|(x, _)| x).max().unwrap();
    let miny = *elves.iter().map(|(_, y)| y).min().unwrap();
    let maxy = *elves.iter().map(|(_, y)| y).max().unwrap();

    (maxx - minx + 1) * (maxy - miny + 1) - elves.len() as i64
}

pub fn part1(s: String) -> Solution {
    let mut elves = parse(s);

    let mut directions = VecDeque::from([0, 2, 3, 1]);
    let mut have_moved;

    for _ in 0..10 {
        (elves, have_moved) = iter_elves(elves, &directions);
        if !have_moved {
            break;
        }
        directions.rotate_left(1);
    }

    Solution::from(score(&elves))
}

pub fn part2(s: String) -> Solution {
    let mut elves = parse(s);

    let mut directions = VecDeque::from([0, 2, 3, 1]);
    let mut have_moved = true;
    let mut i = 0;

    while have_moved {
        i += 1;
        (elves, have_moved) = iter_elves(elves, &directions);
        if !have_moved {
            break;
        }
        directions.rotate_left(1);
    }

    Solution::from(i)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let mut s = HashSet::new();

    for i in 0..input.len() {
        for (j, c) in input[i].chars().enumerate() {
            if c == '#' {
                s.insert((i as i64, j as i64));
            }
        }
    }

    s
}
