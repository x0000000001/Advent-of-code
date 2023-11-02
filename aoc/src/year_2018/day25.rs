use std::collections::{HashMap, HashSet};

use crate::Solution;

type InputType = Vec<Point>;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(i64, i64, i64, i64);
type Vertices<'a> = HashMap<&'a Point, Vec<&'a Point>>;

fn manhattan_distance(p0: &Point, p1: &Point) -> usize {
    ((p0.0 - p1.0).abs() + (p0.1 - p1.1).abs() + (p0.2 - p1.2).abs() + (p0.3 - p1.3).abs()) as usize
}

fn compute_vertices(points: &InputType) -> Vertices {
    let mut vertices = HashMap::from_iter(points.iter().map(|p| (p, vec![])));

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p0 = &points[i];
            let p1 = &points[j];
            let d = manhattan_distance(p0, p1);

            if d > 3 {
                continue;
            }

            let entry0 = vertices.entry(p0).or_insert(vec![]);
            entry0.push(p1);
            let entry1 = vertices.entry(p1).or_insert(vec![]);
            entry1.push(p0);
        }
    }

    vertices
}

fn count_constellations(vertices: Vertices) -> usize {
    let mut count = 0;
    let mut seen = HashSet::new();

    for start_point in vertices.keys().cloned() {
        if seen.contains(start_point) {
            continue;
        }

        count += 1;
        let mut to_see = vec![];

        to_see.push(start_point);

        while let Some(p) = to_see.pop() {
            seen.insert(p);
            for neighbor in vertices.get(p).unwrap() {
                if seen.contains(neighbor) {
                    continue;
                } else {
                    to_see.push(&neighbor);
                }
            }
        }
    }

    count
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(count_constellations(compute_vertices(&input)) as i64)
}

pub fn part2(_: String) -> Solution {
    Solution::Day25Part2
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
            let words = l
                .split(",")
                .map(|w| w.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Point(words[0], words[1], words[2], words[3])
        })
        .collect()
}
