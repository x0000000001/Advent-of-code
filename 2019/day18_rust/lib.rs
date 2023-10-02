use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    hash::Hash,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point(usize, usize);

#[derive(Clone, Copy, Hash)]
pub enum Case {
    Wall,
    Free,
    Key(char),
    Door(char),
}

/// Used as a set, but hashable.
#[derive(PartialEq, Eq, Hash, Clone)]
struct OpenedDoors {
    doors: Vec<char>,
}

impl OpenedDoors {
    fn new() -> OpenedDoors {
        OpenedDoors { doors: vec![] }
    }

    fn count(&self) -> usize {
        return self.doors.len();
    }

    fn add_door(&mut self, door: char) {
        for i in 0..self.doors.len() {
            match self.doors[i].cmp(&door) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => return,
                std::cmp::Ordering::Greater => {
                    self.doors.insert(i + 1, door);
                    return;
                }
            };
        }
    }
}

#[derive(Clone, Hash)]
pub struct Map {
    grid: Vec<Vec<Case>>,
    width: usize,
    height: usize,
    start_points: Vec<Point>,
    keys_count: usize,
    first_key: Point,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    position: Point,
    // this vector has to be alphabetically sorted !
    opened_doors: OpenedDoors,
}

#[derive(Clone)]
struct Vertex {
    neighbor: State,
    distance: usize,
}

struct Graph {
    nodes_by_point: HashMap<Point, Case>,
    nodes: HashMap<Case, Point>,
    vertices: HashMap<State, Vec<Vertex>>,
    computed_keys_combinations: HashSet<OpenedDoors>,
}

#[derive(PartialEq, Eq, Hash)]
struct StateScore {
    state: State,
    length_until_here: usize,
}

impl PartialOrd for StateScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.length_until_here
                .cmp(&other.length_until_here)
                .reverse(),
        )
    }
}

impl Ord for StateScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Graph {
    fn from_map(map: &Map) -> Graph {
        todo!("compute graph with nodes only keys and doors (Djikstra on all the map)");
        // using state as a one element openedKeys here
        let mut seen: HashSet<State> = HashSet::new();
        let mut scores: HashMap<State, usize> = HashMap::new();
        let mut queue: BinaryHeap<StateScore> = BinaryHeap::new();

        while let Some(StateScore {
            state,
            length_until_here,
        }) = queue.pop()
        {
            if seen.contains(&state) {
                continue;
            }
        }

        Graph {
            nodes_by_point: todo!(),
            nodes: todo!(),
            vertices: todo!(),
            computed_keys_combinations: todo!(),
        }
    }

    fn compute_neighbors(&mut self, opened_doors: &OpenedDoors) {
        todo!()
    }

    fn get_neighbors(&mut self, state: &State) -> Vec<Vertex> {
        if !self
            .computed_keys_combinations
            .contains(&state.opened_doors)
        {
            self.compute_neighbors(&state.opened_doors);
        }

        self.vertices
            .get(&state)
            .and_then(|f| Some(f.clone()))
            .unwrap_or(vec![])
    }
}

fn djikstra(mut graph: Graph, start_points: Vec<Point>, keys_count: usize) -> usize {
    let mut queue: BinaryHeap<StateScore> = BinaryHeap::new();
    let mut scores: HashMap<State, usize> = HashMap::new();
    let mut seen: HashSet<State> = HashSet::new();

    for point in start_points {
        let start_state = State {
            position: point,
            opened_doors: OpenedDoors::new(),
        };

        scores.insert(start_state.clone(), 0);

        queue.push(StateScore {
            state: start_state,
            length_until_here: 0,
        });
    }

    while let Some(StateScore {
        state,
        length_until_here,
    }) = queue.pop()
    {
        if state.opened_doors.count() == keys_count {
            return length_until_here;
        }

        if seen.contains(&state) {
            continue;
        }

        for vertex in graph.get_neighbors(&state) {
            let new_score = vertex.distance + length_until_here;

            let entry = scores.entry(vertex.neighbor.clone()).or_insert(usize::MAX);

            if *entry > new_score {
                *entry = new_score;
                queue.push(StateScore {
                    state: vertex.neighbor,
                    length_until_here: new_score,
                });
            }
        }

        seen.insert(state);
    }

    0
}

pub type InputType = Map;

pub fn result_1(map: InputType) -> i64 {
    // let graph = Graph::from_map(&map);
    // djikstra(graph, map.start_points, map.keys_count) as i64
    0
}

pub fn result_2(_map: InputType) -> i64 {
    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let h = contents.lines().count();
    let w = contents.lines().next().unwrap().chars().count();

    let mut map = Map {
        grid: vec![vec![Case::Wall; w]; h],
        width: w,
        height: h,
        start_points: vec![],
        keys_count: 0,
        first_key: Point(0, 0),
    };

    for (x, l) in contents.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            map.grid[x][y] = match c {
                '#' => Case::Wall,
                '.' => Case::Free,
                '@' => {
                    map.start_points.push(Point(x, y));
                    Case::Free
                }
                letter => {
                    if letter.is_uppercase() {
                        Case::Door(letter.to_lowercase().next().unwrap())
                    } else {
                        map.keys_count += 1;
                        map.first_key = Point(x, y);
                        Case::Key(letter)
                    }
                }
            };
        }
    }

    map
}
