#![feature(drain_filter)]
use std::{collections::HashMap, fs};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Wall,
    Free,
    Portal(char),
}

pub type Map = Vec<Vec<State>>;
pub type Position = (usize, usize);

pub type InputType = (Map, HashMap<(char, char), Vec<(usize, usize)>>);

fn djisktra(
    map: &Map,
    portals: &HashMap<char, Vec<(usize, usize)>>,
    start: Position,
    end: Position,
) -> usize {
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

        let mut candidates = vec![];
        let new_score = score + 1;

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

        candidates = candidates
            .into_iter()
            .flat_map(|(cx, cy)| match map[cx][cy] {
                State::Wall => vec![],
                State::Free => vec![(cx, cy); 1],
                State::Portal(c) => portals.get(&c).unwrap().clone(),
            })
            .filter(|p| scores[p.0][p.1] > new_score)
            .collect();

        print!("");

        for (cx, cy) in candidates {
            queue.push((new_score, (cx, cy)));
            scores[cx][cy] = new_score;
        }
    }

    0
}

pub fn result_1((map, portals): InputType) -> i64 {
    let mut start = None;
    let mut end = None;
    let (h, w) = (map.len(), map[0].len());

    for i in 0..h {
        for j in 0..w {
            match map[i][j] {
                State::Portal('A') => {
                    start = Some((i, j));
                }
                State::Portal('Z') => end = Some((i, j)),
                _ => (),
            }
        }
    }

    djisktra(&map, &portals, start.unwrap(), end.unwrap()) as i64
}

pub fn result_2(input: InputType) -> i64 {
    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let (h, w) = (input.len(), input[0].len());
    let mut portals = HashMap::new();
    let mut map = vec![vec![State::Wall; w]; h];

    for i in 0..h {
        for j in 0..w {
            let chars: Vec<char> = input[i].chars().collect();
            map[i][j] = match chars[j] {
                '.' => State::Free,
                '#' => State::Wall,
                ' ' => State::Wall,
                _ => {
                    let entry = portals.entry(chars[j]).or_insert(vec![]);
                    entry.push((i, j));
                    State::Portal(chars[j])
                }
            }
        }
    }

    (map, portals)
}
