#![feature(drain_filter)]
use std::{collections::HashMap, fs};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Wall,
    Free,
    Portal(usize),
}

fn hash_portal(c0: char, c1: char) -> usize {
    let code0 = c0 as usize - 'A' as usize;
    let code1 = c1 as usize - 'A' as usize;

    code0.max(code1) * 25 + code0.min(code1)
}

pub type Map = Vec<Vec<State>>;
pub type Position = (usize, usize);
pub type Portals = HashMap<usize, Vec<((usize, usize), bool)>>;

pub type InputType = (Map, Portals);

fn neighbors(h: usize, w: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut candidates = vec![];

    if x > 0 {
        candidates.push((x - 1, y));
    }
    if y > 0 {
        candidates.push((x, y - 1));
    }
    if x < h - 1 {
        candidates.push((x + 1, y));
    }
    if y < w - 1 {
        candidates.push((x, y + 1));
    }

    candidates
}

fn djisktra(map: &Map, portals: &Portals, start: Position, end: Position) -> usize {
    let (h, w) = (map.len(), map[0].len());
    let mut scores = vec![vec![usize::MAX; w]; h];
    let mut queue = vec![];
    scores[start.0][start.1] = 0;
    queue.push((0, start));

    while !queue.is_empty() {
        queue.sort_by_key(|(score, _)| usize::MAX - score);
        let (score, (x, y)) = queue.pop().unwrap();

        print!("");

        if (x, y) == end {
            return score;
        }

        let mut candidates = neighbors(h, w, x, y);
        let new_score = score + 1;

        candidates = candidates
            .into_iter()
            .flat_map(|(cx, cy)| match map[cx][cy] {
                State::Wall => vec![],
                State::Free => vec![(cx, cy); 1],
                State::Portal(hash) => portals
                    .get(&(hash))
                    .unwrap()
                    .iter()
                    .map(|(p, _)| *p)
                    .collect(),
            })
            .filter(|p| scores[p.0][p.1] > new_score)
            .collect();

        for (cx, cy) in candidates {
            queue.push((new_score, (cx, cy)));
            scores[cx][cy] = new_score;
        }
    }

    0
}

pub fn result_1((map, portals): InputType) -> i64 {
    let start_candidates = portals.get(&hash_portal('A', 'A')).unwrap().clone();
    let end_candidates = portals.get(&hash_portal('Z', 'Z')).unwrap().clone();

    djisktra(&map, &portals, start_candidates[0].0, end_candidates[0].0) as i64
}

fn marsian_djisktra(map: &Map, portals: &Portals, start: Position, end: Position) -> usize {
    let aa_hash = 0;
    let zz_hash = 25 * 25 + 25;
    let (h, w) = (map.len(), map[0].len());
    let mut scores = HashMap::new();
    let mut queue = vec![];
    scores.insert((start, 0), 0);
    queue.push((0, (start, 0)));

    while !queue.is_empty() {
        queue.sort_by_key(|(score, _)| usize::MAX - score);
        let (score, ((x, y), depth)) = queue.pop().unwrap();

        // println!("x {} y {} depth {}", x, y, depth);

        if (x, y) == end && depth == 0 {
            return score;
        }

        let new_score = score + 1;

        let candidates: Vec<((usize, usize), i64)> = neighbors(h, w, x, y)
            .into_iter()
            .flat_map(|(cx, cy)| match map[cx][cy] {
                State::Wall => vec![],
                State::Free => vec![((cx, cy), depth); 1],
                State::Portal(hash) => {
                    if (hash == aa_hash || hash == zz_hash) && depth != 0 {
                        vec![]
                    } else {
                        let mut exits = portals.get(&(hash)).unwrap().clone();
                        // println!("cx,cy {} {} exits {:?}", cx, cy, exits);
                        let is_outer = exits
                            .drain_filter(|((px, py), _)| cx.abs_diff(*px) + cy.abs_diff(*py) < 2)
                            .next()
                            .unwrap()
                            .1;
                        exits
                            .into_iter()
                            .map(|(p, _)| {
                                if is_outer {
                                    (p, depth - 1)
                                } else {
                                    (p, depth + 1)
                                }
                            })
                            .collect()
                    }
                }
            })
            .filter(|&(p, depth)| {
                if depth < 0 {
                    false
                } else if let Some(previous_score) = scores.get(&(p, depth)) {
                    *previous_score > new_score
                } else {
                    true
                }
            })
            .collect();

        for c in candidates {
            queue.push((new_score, c));
            scores.insert(c, new_score);
        }
    }

    0
}

pub fn result_2((map, portals): InputType) -> i64 {
    let start_candidates = portals.get(&hash_portal('A', 'A')).unwrap().clone();
    let end_candidates = portals.get(&hash_portal('Z', 'Z')).unwrap().clone();

    marsian_djisktra(&map, &portals, start_candidates[0].0, end_candidates[0].0) as i64
}

fn portal_neighbor(input: &Vec<Vec<char>>, (x0, y0): Position, (x1, y1): Position) -> Position {
    let (h, w) = (input.len(), input[0].len());

    [(x0, y0), (x1, y1)]
        .into_iter()
        .flat_map(|(x, y)| neighbors(h, w, x, y))
        .filter(|&(x, y)| match input[x][y] {
            '.' => true,
            _ => false,
        })
        .next()
        .unwrap()
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<Vec<char>> = contents
        .lines()
        .into_iter()
        .map(|line| line.to_owned())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let (h, w) = (input.len(), input[0].len());
    let mut portals = HashMap::new();
    let mut map = vec![vec![State::Wall; w]; h];

    for i in 0..h {
        for j in 0..w {
            let c = input[i][j];
            map[i][j] = match c {
                '.' => State::Free,
                '#' => State::Wall,
                ' ' => State::Wall,
                _ => {
                    let (neighbor_pos, neighbor_portal_char) = neighbors(h, w, i, j)
                        .into_iter()
                        .map(|(x, y)| ((x, y), input[x][y]))
                        .filter(|(_, c)| c.is_alphabetic())
                        .next()
                        .unwrap();
                    let hash = hash_portal(c, neighbor_portal_char);
                    let entry = portals.entry(hash).or_insert(vec![]);
                    let is_outer = neighbor_pos.0 == 0
                        || neighbor_pos.1 == 0
                        || i == 0
                        || j == 0
                        || neighbor_pos.0 == h - 1
                        || neighbor_pos.1 == w - 1
                        || i == h - 1
                        || j == w - 1;
                    entry.push((portal_neighbor(&input, (i, j), neighbor_pos), is_outer));
                    State::Portal(hash)
                }
            }
        }
    }

    (map, portals)
}
