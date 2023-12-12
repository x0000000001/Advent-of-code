mod helpers;

use core::panic;
use std::collections::HashMap;

pub use helpers::Solution;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Pipe(Dir, Dir),
    Floor,
    Start,
}

impl Shape {
    fn contains(&self, d: &Dir) -> bool {
        match self {
            Shape::Pipe(d0, d1) => d0 == d || d1 == d,
            Shape::Floor => false,
            Shape::Start => false,
        }
    }

    fn get_other(&self, d: &Dir) -> Dir {
        match self {
            Shape::Pipe(d0, d1) if d0 == d => *d1,
            Shape::Pipe(d0, d1) if d1 == d => *d0,
            _ => panic!(),
        }
    }
}

type Pos = (usize, usize);
type Map = Vec<Vec<Shape>>;
type InputType = (Map, Pos);

fn get_start_pos(start: (usize, usize), map: &Map) -> (Vec<Pos>, Vec<Dir>, Shape) {
    let mut starts_pos = vec![];
    let mut starts_from_dirs = vec![];

    let (h, w) = (map.len(), map[0].len());
    if start.0 < (h - 1) {
        if let pipe @ Shape::Pipe(_, _) = map[start.0 + 1][start.1] {
            if pipe.contains(&Dir::North) {
                starts_pos.push((start.0 + 1, start.1));
                starts_from_dirs.push(Dir::North);
            }
        }
    }
    if start.0 > 0 {
        if let pipe @ Shape::Pipe(_, _) = map[start.0 - 1][start.1] {
            if pipe.contains(&Dir::South) {
                starts_pos.push((start.0 - 1, start.1));
                starts_from_dirs.push(Dir::South);
            }
        }
    }
    if start.1 < (w - 1) {
        if let pipe @ Shape::Pipe(_, _) = map[start.0][start.1 + 1] {
            if pipe.contains(&Dir::West) {
                starts_pos.push((start.0, start.1 + 1));
                starts_from_dirs.push(Dir::West);
            }
        }
    }
    if start.1 > 0 {
        if let pipe @ Shape::Pipe(_, _) = map[start.0][start.1 - 1] {
            if pipe.contains(&Dir::East) {
                starts_pos.push((start.0, start.1 - 1));
                starts_from_dirs.push(Dir::East);
            }
        }
    }

    let start_type =
        if starts_from_dirs.contains(&Dir::South) && starts_from_dirs.contains(&Dir::North) {
            Shape::Pipe(Dir::South, Dir::North)
        } else if starts_from_dirs.contains(&Dir::East) && starts_from_dirs.contains(&Dir::West) {
            Shape::Pipe(Dir::East, Dir::West)
        } else if starts_from_dirs.contains(&Dir::South) && starts_from_dirs.contains(&Dir::West) {
            Shape::Pipe(Dir::North, Dir::East)
        } else if starts_from_dirs.contains(&Dir::North) && starts_from_dirs.contains(&Dir::West) {
            Shape::Pipe(Dir::South, Dir::East)
        } else if starts_from_dirs.contains(&Dir::South) && starts_from_dirs.contains(&Dir::East) {
            Shape::Pipe(Dir::North, Dir::West)
        } else if starts_from_dirs.contains(&Dir::North) && starts_from_dirs.contains(&Dir::East) {
            Shape::Pipe(Dir::South, Dir::West)
        } else {
            panic!()
        };

    assert_eq!(starts_pos.len(), 2);

    (starts_pos, starts_from_dirs, start_type)
}

fn get_path(start: Pos, map: &Map) -> Vec<Pos> {
    let (starts_pos, starts_from_dirs, _) = get_start_pos(start, map);
    let mut pos = starts_pos[0];
    let mut from_dir = starts_from_dirs[0];
    let mut shape = map[pos.0][pos.1];
    let mut path = vec![];

    while shape != Shape::Start {
        path.push(pos);

        let next_dir = shape.get_other(&from_dir);

        match next_dir {
            Dir::North => {
                pos.0 -= 1;
                from_dir = Dir::South;
            }
            Dir::South => {
                pos.0 += 1;
                from_dir = Dir::North;
            }
            Dir::East => {
                pos.1 += 1;
                from_dir = Dir::West;
            }
            Dir::West => {
                pos.1 -= 1;
                from_dir = Dir::East;
            }
        }

        shape = map[pos.0][pos.1];
    }

    path.push(pos);

    path
}

pub fn part1(s: String) -> Solution {
    let (map, start) = parse(s);
    let path = get_path(start, &map);

    Solution::from(path.len() as u64 / 2)
}

pub fn part2(s: String) -> Solution {
    part2_smart(s)
}

/// Mathematical optimization using shoelace formula and
/// Pick's theoreme
/// https://en.wikipedia.org/wiki/Shoelace_formula
/// https://fr.wikipedia.org/wiki/Th%C3%A9or%C3%A8me_de_Pick
fn part2_smart(s: String) -> Solution {
    // TODO determine orientation
    let (map, start) = parse(s);
    let mut path = get_path(start, &map);

    loop {
        let mut area: i64 = 0;
        for i in 0..path.len() {
            area += (path[i].1 as i64 + path[(i + 1) % path.len()].1 as i64)
                * (path[i].0 as i64 - path[(i + 1) % path.len()].0 as i64);
        }

        area /= 2;

        let inside_points = area + 1 - (path.len() as i64 / 2);

        if inside_points > 0 {
            return Solution::from(inside_points);
        }

        path.reverse();
    }
}

fn parse(s: String) -> InputType {
    let mut start = (0, 0);

    (
        s.lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '|' => Shape::Pipe(Dir::South, Dir::North),
                        '-' => Shape::Pipe(Dir::East, Dir::West),
                        'L' => Shape::Pipe(Dir::North, Dir::East),
                        'J' => Shape::Pipe(Dir::North, Dir::West),
                        'F' => Shape::Pipe(Dir::South, Dir::East),
                        '7' => Shape::Pipe(Dir::South, Dir::West),
                        '.' => Shape::Floor,
                        'S' => {
                            start = (i, j);
                            Shape::Start
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect(),
        start,
    )
}

#[allow(dead_code)]
fn get_distances(start: (usize, usize), map: &Map) -> (HashMap<Pos, usize>, Shape) {
    let (starts_pos, starts_from_dirs, start_type) = get_start_pos(start, map);
    let mut distances: HashMap<Pos, usize> = HashMap::new();
    distances.insert(start, 0);

    for i in 0..starts_pos.len() {
        propagate(starts_pos[i], starts_from_dirs[i], map, &mut distances);
    }

    (distances, start_type)
}

#[allow(dead_code)]
fn propagate(mut pos: Pos, mut from_dir: Dir, map: &Map, distances: &mut HashMap<Pos, usize>) {
    let mut i = 1;
    let mut shape = map[pos.0][pos.1];

    while shape != Shape::Start {
        let entry = distances.entry(pos).or_insert(usize::MAX);
        *entry = (*entry).min(i);

        let next_dir = shape.get_other(&from_dir);

        match next_dir {
            Dir::North => {
                pos.0 -= 1;
                from_dir = Dir::South;
            }
            Dir::South => {
                pos.0 += 1;
                from_dir = Dir::North;
            }
            Dir::East => {
                pos.1 += 1;
                from_dir = Dir::West;
            }
            Dir::West => {
                pos.1 -= 1;
                from_dir = Dir::East;
            }
        }

        shape = map[pos.0][pos.1];
        i += 1;
    }
}

#[allow(dead_code)]
fn part2_original(s: String) -> Solution {
    let (mut map, start) = parse(s);
    let (distances, start_type) = get_distances(start, &map);

    let (_, w) = (map.len(), map[0].len());
    map[start.0][start.1] = start_type;

    let mut inside_count = 0;

    for (i, line) in map.iter().enumerate() {
        let mut is_inside = false;
        let mut last_corner: Option<Dir> = None;

        for (j, shape) in line.iter().enumerate() {
            if distances.contains_key(&(i, j)) {
                match shape {
                    Shape::Pipe(Dir::South, Dir::North) => {
                        is_inside = !is_inside;
                    }
                    Shape::Pipe(Dir::East, Dir::West) => (),
                    Shape::Pipe(vert_dir, _) => {
                        if let Some(last_dir) = last_corner {
                            if last_dir != *vert_dir {
                                is_inside = !is_inside;
                            }
                            last_corner = None;
                        } else if j < (w - 1)
                            && map[i][j + 1] != Shape::Pipe(Dir::South, Dir::North)
                        {
                            last_corner = Some(*vert_dir);
                        } else {
                            is_inside = !is_inside;
                        }
                    }
                    _ => (),
                }
            } else if is_inside {
                inside_count += 1;
            }
        }
    }

    Solution::from(inside_count)
}
