use crate::Solution;
use std::collections::{HashMap, HashSet};

type InputType = Regex;

#[derive(Debug)]
pub enum Regex {
    List(Vec<Regex>),
    Choice(Vec<Regex>),
    Leaf(Vec<char>),
}

struct Map {
    points: HashSet<(i64, i64)>,
    doors: HashSet<((i64, i64), (i64, i64))>,
}

impl Regex {
    #[allow(dead_code)]
    fn furhest_room_distance(&self) -> usize {
        match self {
            Regex::List(children) => children.iter().map(|c| c.furhest_room_distance()).sum(),
            Regex::Choice(children) => children
                .iter()
                .map(|c| c.furhest_room_distance())
                .max()
                .unwrap(),
            Regex::Leaf(chars) => chars.len(),
        }
    }

    #[allow(dead_code)]
    fn count_possible_max_paths(&self) -> usize {
        match self {
            Regex::List(children) => children
                .iter()
                .map(|c| c.count_possible_max_paths())
                .product(),
            Regex::Choice(children) => children.iter().map(|c| c.count_possible_max_paths()).sum(),
            Regex::Leaf(_) => 1,
        }
    }
}

#[allow(dead_code)]
impl Map {
    fn compute_map(regex: &Regex, mut position: (i64, i64), map: &mut Map) -> HashSet<(i64, i64)> {
        // TODO this doesn't seem to work
        // is parsing bad ?
        map.points.insert(position);

        match regex {
            Regex::List(children) => {
                let mut current_possible_ends = HashSet::new();
                current_possible_ends.insert(position);

                for c in children {
                    current_possible_ends = current_possible_ends
                        .into_iter()
                        .map(|end| Map::compute_map(c, end, map))
                        .flatten()
                        .collect();
                }

                current_possible_ends
            }
            Regex::Choice(children) => children
                .iter()
                .map(|c| Map::compute_map(c, position, map))
                .flatten()
                .collect(),
            Regex::Leaf(chars) => {
                for c in chars {
                    let next = match c {
                        'N' => (position.0, position.1 - 1),
                        'S' => (position.0, position.1 + 1),
                        'W' => (position.0 - 1, position.1),
                        'E' => (position.0 + 1, position.1),
                        _ => panic!(),
                    };

                    map.doors.insert((position, next));
                    map.doors.insert((next, position));
                    position = next;

                    map.points.insert(position);
                }

                HashSet::from([position])
            }
        }
    }

    fn from(regex: &Regex) -> Map {
        let mut map = Map {
            points: HashSet::new(),
            doors: HashSet::new(),
        };

        Map::compute_map(regex, (0, 0), &mut map);

        map
    }

    #[allow(dead_code)]
    fn print(&self) {
        let minx = self.points.iter().min_by_key(|p| p.0).unwrap().0;
        let miny = self.points.iter().min_by_key(|p| p.1).unwrap().1;
        let maxx = self.points.iter().max_by_key(|p| p.0).unwrap().0;
        let maxy = self.points.iter().max_by_key(|p| p.1).unwrap().1;

        for y in 0..=(2 * (maxy - miny) + 2) {
            for x in 0..=(2 * (maxx - minx) + 2) {
                let square_pos = (x / 2 + minx, y / 2 + miny);

                match (x % 2, y % 2) {
                    (1, 1) => {
                        print!(
                            "{}",
                            if square_pos.0 == 0 && square_pos.1 == 0 {
                                "X"
                            } else if self.points.contains(&square_pos) {
                                "."
                            } else {
                                "#"
                            }
                        )
                    }
                    (0, 0) => {
                        print!("#",)
                    }
                    (1, 0) => {
                        let below = (square_pos.0, square_pos.1 - 1);
                        print!(
                            "{}",
                            if self.doors.contains(&(below, square_pos)) {
                                "-"
                            } else {
                                "#"
                            }
                        )
                    }
                    (0, 1) => {
                        let left = (square_pos.0 - 1, square_pos.1);
                        print!(
                            "{}",
                            if self.doors.contains(&(left, square_pos)) {
                                "|"
                            } else {
                                "#"
                            }
                        )
                    }
                    _ => panic!(),
                }
            }

            println!();
        }
    }

    #[allow(dead_code)]
    fn furthest_room_distance(&self) -> usize {
        let mut seen = HashSet::new();
        let mut scores = HashMap::new();
        let mut queue = vec![];
        scores.insert((0, 0), 0);
        queue.push((0, 0));
        let mut worst_score = 0;

        while let Some(position @ (x, y)) = queue.pop() {
            let score = *scores.get(&position).unwrap();
            seen.insert(position);
            if score > worst_score {
                worst_score = score;
            }

            let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|n| self.doors.contains(&(*n, position)) && !seen.contains(n));

            for neighbor in neighbors {
                if let Some(&previous_score) = scores.get(&neighbor) {
                    if previous_score < score + 1 {
                        continue;
                    }
                }

                scores.insert(neighbor, score + 1);
                queue.push(neighbor);
            }
        }

        worst_score
    }
}

pub fn part1(s: String) -> Solution {
    // let regex = parse(s);

    // let map = Map::from(&regex);

    let line: Vec<char> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .next()
        .unwrap()
        .chars()
        .collect();

    Solution::from(solve(line).0 as i64)
}

pub fn part2(s: String) -> Solution {
    // let regex = parse(s);

    // let map = Map::from(&regex);

    let line: Vec<char> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .next()
        .unwrap()
        .chars()
        .collect();

    Solution::from(solve(line).1 as i64)
}

fn parse_regex(s: &Vec<char>, start: usize, end: usize) -> Regex {
    assert!(start <= end);

    if s[start] == '(' {
        assert_eq!(s[end], ')');

        let mut parentheses_count = 0;
        let mut separations_indices = vec![];

        for i in (start + 1)..end {
            match s[i] {
                '(' => parentheses_count += 1,
                ')' => parentheses_count -= 1,
                '|' => {
                    if parentheses_count == 0 {
                        separations_indices.push(i)
                    }
                }
                _ => (),
            }
        }

        assert_eq!(parentheses_count, 0);

        let mut choices = vec![];

        choices.push(parse_regex(s, start + 1, separations_indices[0] - 1));

        for i in 1..separations_indices.len() - 1 {
            choices.push(parse_regex(
                s,
                separations_indices[i] + 1,
                separations_indices[i + 1] - 1,
            ))
        }

        if s[end - 1] != '|' {
            choices.push(parse_regex(
                s,
                *separations_indices.last().unwrap() + 1,
                end - 1,
            ));
        } else {
            choices.push(Regex::Leaf(vec![]))
        }

        Regex::Choice(choices)
    } else {
        let mut parentheses_count = 0;
        let mut separations_indices = vec![];

        for i in start..=end {
            match s[i] {
                '(' => {
                    if parentheses_count == 0 {
                        separations_indices.push(i);
                    }
                    parentheses_count += 1;
                }
                ')' => {
                    parentheses_count -= 1;
                    if parentheses_count == 0 {
                        separations_indices.push(i);
                    }
                }
                _ => (),
            }
        }

        assert_eq!(parentheses_count, 0);

        if separations_indices.len() == 0 {
            return Regex::Leaf(s[start..=end].iter().cloned().collect());
        }

        let mut children = vec![];

        if s[start] != '(' {
            children.push(parse_regex(s, start, separations_indices[0] - 1));
        }

        for i in 0..separations_indices.len() - 1 {
            if separations_indices[i] == separations_indices[i + 1] - 1 {
                continue;
            }

            if s[separations_indices[i]] == '(' {
                assert_eq!(s[separations_indices[i + 1]], ')');

                children.push(parse_regex(
                    s,
                    separations_indices[i],
                    separations_indices[i + 1],
                ))
            } else {
                assert_eq!(s[separations_indices[i + 1]], '(');

                children.push(parse_regex(
                    s,
                    separations_indices[i] + 1,
                    separations_indices[i + 1] - 1,
                ))
            }
        }

        if s[end] != ')' {
            children.push(parse_regex(
                s,
                *separations_indices.last().unwrap() + 1,
                end,
            ));
        }

        Regex::List(children)
    }
}

fn solve(s: Vec<char>) -> (usize, usize) {
    let mut queue = vec![];
    let mut distances = HashMap::new();
    let (mut x, mut y) = (0, 0);
    distances.insert((0, 0), 0);

    for c in s {
        let distance = *distances.get(&(x, y)).unwrap();

        match c {
            '^' | '$' => (),
            '(' => queue.push((x, y)),
            ')' => (x, y) = queue.pop().unwrap(),
            '|' => (x, y) = *queue.last().unwrap(),
            dir => {
                (x, y) = match dir {
                    'N' => (x, y - 1),
                    'S' => (x, y + 1),
                    'W' => (x - 1, y),
                    'E' => (x + 1, y),
                    _ => panic!(),
                };

                if let Some(&previous_d) = distances.get(&(x, y)) {
                    if previous_d > distance + 1 {
                        distances.insert((x, y), distance + 1);
                    }
                } else {
                    distances.insert((x, y), distance + 1);
                }
            }
        }
    }

    (
        *distances.values().max().unwrap(),
        distances.values().filter(|&&v| v >= 1000).count(),
    )
}

#[allow(dead_code)]
fn parse(s: String) -> InputType {
    let line: Vec<char> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .next()
        .unwrap()
        .chars()
        .collect();

    parse_regex(&line, 1, line.len() - 2)
}
