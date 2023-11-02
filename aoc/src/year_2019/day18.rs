use crate::Solution;

use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    ops::{Index, IndexMut},
};

type InputType = Map;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Point(usize, usize);

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Case {
    Wall,
    Free,
    Key(char),
    Door(char),
    Start,
}

#[derive(Clone, Hash, Debug)]
pub struct Map {
    grid: Vec<Vec<Case>>,
    width: usize,
    height: usize,
    start_points: Vec<Point>,
    keys_positions: Vec<Point>,
}

const ASCII_LOWERCASE_A: u8 = 97;
const ASCII_LOWERCASE_Z: u8 = 122;

/// Used as a set, but hashable.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Doors {
    bits: usize,
}

impl Doors {
    fn new() -> Doors {
        Doors { bits: 0 }
    }

    fn count(&self) -> usize {
        usize::count_ones(self.bits) as usize
    }

    fn add_door(&mut self, door: char) {
        self.bits |= 1 << (door as u32 - ASCII_LOWERCASE_A as u32);
    }

    fn contains(&self, other: &Doors) -> bool {
        (!self.bits & other.bits) == 0
    }

    #[allow(dead_code)]
    fn remove(&mut self, door: char) {
        self.bits &= !(1 << (door as u32 - ASCII_LOWERCASE_A as u32));
    }
}

impl IndexMut<&Point> for Map {
    fn index_mut(&mut self, p: &Point) -> &mut Self::Output {
        &mut self.grid[p.0][p.1]
    }
}

impl Index<&Point> for Map {
    type Output = Case;

    fn index(&self, p: &Point) -> &Self::Output {
        &self.grid[p.0][p.1]
    }
}

impl Map {
    fn get_neighbors(&self, p: &Point) -> Vec<(Point, Case)> {
        let mut neighbors = vec![];

        if p.0 > 0 {
            neighbors.push(Point(p.0 - 1, p.1));
        }
        if p.0 < self.height - 1 {
            neighbors.push(Point(p.0 + 1, p.1));
        }
        if p.1 > 0 {
            neighbors.push(Point(p.0, p.1 - 1));
        }
        if p.1 < self.width - 1 {
            neighbors.push(Point(p.0, p.1 + 1));
        }

        neighbors
            .into_iter()
            .filter_map(|p| match self[&p] {
                Case::Wall => None,
                case => Some((p, case)),
            })
            .collect()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    keys: Vec<char>,
    opened_doors: Doors,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Vertex {
    neighbor_key: char,
    distance: usize,
    needed_doors: Doors,
}

struct Graph {
    keys: HashSet<char>,
    vertices: HashMap<char, HashSet<Vertex>>,
    start_state: State,
}

/// (key, distance to key, doors between key and start)
fn djisktra_for_node(map: &Map, start: Point) -> HashSet<Vertex> {
    let mut seen: HashMap<Point, HashSet<Doors>> = HashMap::new();
    let mut distances: HashMap<(Point, Doors), usize> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut reachable_keys = HashSet::new();

    let start_node = (start, Doors::new());
    queue.push_back(start_node.clone());
    distances.insert(start_node, 0);

    while let Some(node @ (point, doors)) = queue.pop_front() {
        let distance = *distances.get(&node).unwrap();

        // we don't want to explore a path that requires more doors
        // to go in and is longer than a path with less doors to open
        // and is shorter or equal in distance
        if let Some(seen_doors_vec) = seen.get(&point) {
            if seen_doors_vec.iter().fold(false, |acc, seen_doors| {
                acc || {
                    if doors.contains(seen_doors) {
                        match distances.get(&(point, *seen_doors)) {
                            Some(&distance_with_less_doors) => distance_with_less_doors <= distance,
                            None => panic!(),
                        }
                    } else {
                        false
                    }
                }
            }) {
                continue;
            }
        }

        let entry = seen.entry(point).or_insert(HashSet::new());
        entry.insert(doors);

        if point != start {
            match map[&point] {
                Case::Key(c) => {
                    reachable_keys.insert(Vertex {
                        neighbor_key: c,
                        distance,
                        needed_doors: doors.clone(),
                    });
                }
                _ => (),
            }
        }

        for (neighbor_pos, neighbor_case) in map.get_neighbors(&point) {
            let mut neighbor_doors = doors.clone();

            if let Case::Door(c) = neighbor_case {
                neighbor_doors.add_door(c);
            }

            let neighbor_node = (neighbor_pos, neighbor_doors);

            if let Some(&previous_distance) = distances.get(&neighbor_node) {
                if previous_distance <= distance + 1 {
                    continue;
                }
            }

            distances.insert(neighbor_node.clone(), distance + 1);
            queue.push_back(neighbor_node);
        }
    }

    reachable_keys
}

impl Graph {
    fn from_map(map: &Map) -> Graph {
        let mut graph = Graph {
            keys: HashSet::from_iter(map.keys_positions.iter().map(|p| match map[p] {
                Case::Key(c) => c,
                _ => panic!(),
            })),
            start_state: State {
                keys: map
                    .start_points
                    .iter()
                    .enumerate()
                    .map(|(i, _)| (ASCII_LOWERCASE_Z + 1 + i as u8) as char)
                    .collect(),
                opened_doors: Doors::new(),
            },
            vertices: HashMap::new(),
        };

        // distances from starts
        for (i, start_pos) in map.start_points.iter().enumerate() {
            let c = (ASCII_LOWERCASE_Z + 1 + i as u8) as char;

            graph.vertices.insert(c, djisktra_for_node(map, *start_pos));
        }

        // distances keys to keys
        for key_pos in map.keys_positions.iter() {
            let c = match map[key_pos] {
                Case::Key(c) => c,
                _ => panic!(),
            };

            graph.vertices.insert(c, djisktra_for_node(map, *key_pos));
        }

        graph
    }

    /// (neighbor state, distance)
    fn get_neighbors(&mut self, state: &State) -> Vec<(State, usize)> {
        let mut neighbors = vec![];

        for (i, key) in state.keys.iter().enumerate() {
            let accessible_keys: Vec<(char, usize)> = self
                .vertices
                .get(&key)
                .unwrap()
                .iter()
                .filter_map(
                    |Vertex {
                         neighbor_key,
                         distance,
                         needed_doors,
                     }| {
                        if state.opened_doors.contains(needed_doors) {
                            Some((*neighbor_key, *distance))
                        } else {
                            None
                        }
                    },
                )
                .collect();

            for (next_key, distance) in accessible_keys {
                let mut new_state = state.clone();
                new_state.keys[i] = next_key;
                new_state.opened_doors.add_door(next_key);
                neighbors.push((new_state, distance));
            }
        }

        neighbors
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct StateScore {
    state: State,
    distance: usize,
}

impl PartialOrd for StateScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance).reverse())
    }
}

impl Ord for StateScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn djikstra(mut graph: Graph) -> usize {
    let mut queue: BinaryHeap<StateScore> = BinaryHeap::new();
    let mut distances: HashMap<State, usize> = HashMap::new();
    let mut seen: HashSet<State> = HashSet::new();

    distances.insert(graph.start_state.clone(), 0);

    queue.push(StateScore {
        state: graph.start_state.clone(),
        distance: 0,
    });

    while let Some(StateScore { state, distance }) = queue.pop() {
        if state.opened_doors.count() == graph.keys.len() {
            return distance;
        }

        if seen.contains(&state) {
            continue;
        }

        seen.insert(state.clone());

        for (candidate_state, neighbor_distance) in graph.get_neighbors(&state) {
            let candidate_score = distance + neighbor_distance;

            let entry = distances
                .entry(candidate_state.clone())
                .or_insert(usize::MAX);

            if *entry > candidate_score {
                *entry = candidate_score;
                queue.push(StateScore {
                    state: candidate_state,
                    distance: candidate_score,
                });
            }
        }
    }

    0
}

pub fn part1(s: String) -> Solution {
    let map = parse(s);

    let graph = Graph::from_map(&map);

    Solution::from(djikstra(graph) as i64)
}

pub fn part2(s: String) -> Solution {
    let mut map = parse(s);

    assert_eq!(map.start_points.len(), 1);

    let start = map.start_points.pop().unwrap();

    for new_start in [
        Point(start.0 - 1, start.1 - 1),
        Point(start.0 + 1, start.1 - 1),
        Point(start.0 - 1, start.1 + 1),
        Point(start.0 + 1, start.1 + 1),
    ] {
        map[&new_start] = Case::Start;
        map.start_points.push(new_start);
    }

    for new_wall in [
        start,
        Point(start.0 - 1, start.1),
        Point(start.0 + 1, start.1),
        Point(start.0, start.1 - 1),
        Point(start.0, start.1 + 1),
    ] {
        map[&new_wall] = Case::Wall;
    }

    let graph = Graph::from_map(&map);

    Solution::from(djikstra(graph) as i64)
}

fn parse(s: String) -> InputType {
    let h = s.lines().count();
    let w = s.lines().next().unwrap().chars().count();

    let mut map = Map {
        grid: vec![vec![Case::Wall; w]; h],
        width: w,
        height: h,
        start_points: vec![],
        keys_positions: vec![],
    };

    for (x, l) in s.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            let pos = Point(x, y);

            map[&pos] = match c {
                '#' => Case::Wall,
                '.' => Case::Free,
                '@' => {
                    map.start_points.push(Point(x, y));
                    Case::Start
                }
                letter => {
                    if letter.is_uppercase() {
                        Case::Door(letter.to_lowercase().next().unwrap())
                    } else {
                        map.keys_positions.push(pos);
                        Case::Key(letter)
                    }
                }
            };
        }
    }

    map
}
