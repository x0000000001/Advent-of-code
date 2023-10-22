use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    hash::Hash,
    ops,
};

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

/// Used as a set, but hashable.
#[derive(Clone, Debug)]
struct OpenedDoors {
    doors: Vec<char>,
    order: Vec<char>,
}

impl Hash for OpenedDoors {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.doors.hash(state);
    }
}

impl PartialEq for OpenedDoors {
    fn eq(&self, other: &Self) -> bool {
        self.doors.cmp(&other.doors) == std::cmp::Ordering::Equal
    }
}

impl Eq for OpenedDoors {}

impl OpenedDoors {
    fn new() -> OpenedDoors {
        OpenedDoors {
            doors: vec![],
            order: vec![],
        }
    }

    fn count(&self) -> usize {
        return self.doors.len();
    }

    fn add_door(&mut self, door: char) {
        let count = self.count();

        for i in 0..(count + 1) {
            if i == count {
                self.doors.push(door);
                break;
            } else {
                match self.doors[i].cmp(&door) {
                    std::cmp::Ordering::Less => (),
                    std::cmp::Ordering::Equal => panic!("door already opened"),
                    std::cmp::Ordering::Greater => {
                        self.doors.insert(i + 1, door);
                        break;
                    }
                }
            }
        }

        self.order.push(door);
    }

    fn remove_last_opened(&mut self) -> char {
        if let Some(door) = self.order.pop() {
            for i in 0..self.doors.len() {
                match self.doors[i].cmp(&door) {
                    std::cmp::Ordering::Less => (),
                    std::cmp::Ordering::Equal => {
                        self.doors.remove(i);
                        return door;
                    }
                    std::cmp::Ordering::Greater => (),
                };
            }

            panic!("doors ordered and unordered weren't coherent")
        } else {
            panic!("no door to remove")
        }
    }
}

#[derive(Clone, Hash, Debug)]
pub struct Map {
    grid: Vec<Vec<Case>>,
    width: usize,
    height: usize,
    start_points: Vec<Point>,
    keys_count: usize,
}

impl ops::Index<Point> for Map {
    type Output = Case;

    fn index(&self, point: Point) -> &Self::Output {
        &self.grid[point.0][point.1]
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    position: Point,
    opened_doors: OpenedDoors,
}

#[derive(Clone, Debug)]
struct Vertex {
    neighbor: Point,
    distance: usize,
}

struct Graph {
    nodes_by_point: HashMap<Point, Case>,
    nodes: HashMap<Case, Point>,
    vertices: HashMap<State, HashMap<Case, Vertex>>,
    start_states: Vec<State>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct GraphNode {
    position: Point,
    from_position: Point,
    from_case: Case,
}

impl Graph {
    fn from_map(map: &Map) -> Graph {
        let mut graph = Graph {
            nodes_by_point: HashMap::new(),
            nodes: HashMap::new(),
            vertices: HashMap::new(),
            start_states: vec![],
        };

        // we explore the whole map, so no need for priority
        let mut seen: HashSet<GraphNode> = HashSet::new();
        let mut lengths: HashMap<GraphNode, usize> = HashMap::new();
        let mut queue: Vec<(GraphNode, usize)> = vec![];

        for start_point in map.start_points.clone() {
            let start_node = GraphNode {
                position: start_point,
                from_position: start_point,
                from_case: Case::Start,
            };

            let start_state = State {
                position: start_point.clone(),
                opened_doors: OpenedDoors::new(),
            };

            // lengths.insert(start_node.clone(), 0);
            queue.push((start_node.clone(), 0));

            graph.start_states.push(start_state.clone());
            graph
                .nodes_by_point
                .insert(start_point.clone(), Case::Start);
            graph.nodes.insert(Case::Start, start_point);
            graph.vertices.insert(start_state.clone(), HashMap::new());
        }

        while let Some((
            current_node @ GraphNode {
                position: Point(x, y),
                from_position,
                from_case,
            },
            length,
        )) = queue.pop()
        {
            if seen.contains(&current_node) {
                continue;
            }

            let mut neighbors = vec![];
            if x > 0 {
                neighbors.push(Point(x - 1, y));
            }
            if y > 0 {
                neighbors.push(Point(x, y - 1));
            }
            if x < map.height {
                neighbors.push(Point(x + 1, y));
            }
            if y < map.width {
                neighbors.push(Point(x, y + 1));
            }

            for candidate_position in neighbors.into_iter().filter(|p| match map[*p] {
                Case::Wall => false,
                _ => true,
            }) {
                let mut candidate_node = GraphNode {
                    position: candidate_position,
                    from_position: from_position,
                    from_case: from_case,
                };

                let mut candidate_length = length + 1;

                if seen.contains(&candidate_node) {
                    continue;
                }

                if let Some(previous_length) = lengths.get(&candidate_node) {
                    if *previous_length <= length + 1 {
                        continue;
                    }
                }

                match map[candidate_position] {
                    case @ Case::Door(_) | case @ Case::Key(_) | case @ Case::Start => {
                        graph.nodes.insert(case, candidate_position);
                        graph.nodes_by_point.insert(candidate_position, case);

                        let candidate_state = State {
                            position: candidate_position,
                            opened_doors: OpenedDoors::new(),
                        };

                        let candidate_entry = graph
                            .vertices
                            .entry(candidate_state)
                            .or_insert(HashMap::new());

                        candidate_entry.insert(
                            from_case,
                            Vertex {
                                neighbor: from_position,
                                distance: length + 1,
                            },
                        );

                        candidate_node.from_position = candidate_position;
                        candidate_node.from_case = case;
                        candidate_length = 0;
                    }
                    _ => (),
                }

                queue.push((candidate_node, candidate_length));
                lengths.insert(candidate_node, candidate_length);
            }

            seen.insert(current_node);
        }

        graph
    }

    fn get_neighbors(&mut self, state: &State, include_doors: bool) -> Vec<(Case, Vertex)> {
        if !self.vertices.contains_key(&state) {
            let mut previous_doors = state.opened_doors.clone();
            let last_opened_door = previous_doors.remove_last_opened();

            let previous_state = State {
                position: state.position.clone(),
                opened_doors: previous_doors.clone(),
            };

            let previous_neighbors = self.get_neighbors(&previous_state, true);

            let mut new_neighbors: HashMap<Case, Vertex> = HashMap::new();

            for (vertex_case, vertex) in previous_neighbors {
                match vertex_case {
                    Case::Door(c) | Case::Key(c) if c == last_opened_door => {
                        // removed door leads to new neighbors
                        let removed_case_state = State {
                            position: vertex.neighbor,
                            opened_doors: previous_doors.clone(),
                        };

                        for (removed_case_neighbor_case, removed_case_neighbor) in
                            self.get_neighbors(&removed_case_state, true)
                        {
                            // don't add node neighbor to itself
                            if removed_case_neighbor.neighbor == state.position {
                                continue;
                            }

                            let candidate_vertex = Vertex {
                                neighbor: removed_case_neighbor.neighbor,
                                distance: removed_case_neighbor.distance + vertex.distance,
                            };

                            if let Some(previous_vertex) =
                                new_neighbors.get(&removed_case_neighbor_case)
                            {
                                if previous_vertex.distance <= candidate_vertex.distance {
                                    continue;
                                }
                            }

                            new_neighbors.insert(removed_case_neighbor_case, candidate_vertex);
                        }
                    }
                    _ => {
                        new_neighbors.insert(vertex_case, vertex.clone());
                    }
                }
            }

            self.vertices.insert(state.clone(), new_neighbors);
        }

        if include_doors {
            self.vertices
                .get(&state)
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        } else {
            self.vertices
                .get(&state)
                .unwrap()
                .iter()
                .filter(|(k, _)| match k {
                    Case::Door(_) => false,
                    _ => true,
                })
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        }
    }
}

fn djikstra(mut graph: Graph, keys_count: usize) -> usize {
    let mut queue: BinaryHeap<StateScore> = BinaryHeap::new();
    let mut scores: HashMap<State, usize> = HashMap::new();
    let mut seen: HashSet<State> = HashSet::new();

    for start_state in graph.start_states.iter() {
        scores.insert(start_state.clone(), 0);

        queue.push(StateScore {
            state: start_state.clone(),
            length_until_here: 0,
        });
    }

    while let Some(StateScore {
        state,
        length_until_here,
    }) = queue.pop()
    {
        // println!("{:?} {}", state, length_until_here);
        if state.opened_doors.count() == keys_count {
            return length_until_here;
        }

        if seen.contains(&state) {
            continue;
        }

        for (vertex_case, vertex) in graph.get_neighbors(&state, false) {
            let candidate_score = length_until_here + vertex.distance;
            let mut candidate_opened_doors = state.opened_doors.clone();

            match vertex_case {
                Case::Key(c) => {
                    candidate_opened_doors.add_door(c);
                }
                __ => (),
            };

            let candidate_state = State {
                position: vertex.neighbor,
                opened_doors: candidate_opened_doors,
            };

            let entry = scores.entry(candidate_state.clone()).or_insert(usize::MAX);

            if *entry > candidate_score {
                *entry = candidate_score;
                queue.push(StateScore {
                    state: candidate_state,
                    length_until_here: candidate_score,
                });
            }
        }

        seen.insert(state);
    }

    0
}

pub type InputType = Map;

pub fn result_1(map: InputType) -> i64 {
    let graph = Graph::from_map(&map);
    for s in graph.vertices.clone().into_iter().map(|(state, vs)| {
        let mut s = format!("{} {} : ", state.position.0, state.position.1);
        for (c, v) in vs {
            s += &format!("\n{} {} ({:?})", v.neighbor.0, v.neighbor.1, c);
        }
        s += "\n";
        s
    }) {
        println!("{s}");
    }

    djikstra(graph, map.keys_count) as i64
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
    };

    for (x, l) in contents.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            map.grid[x][y] = match c {
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
                        map.keys_count += 1;
                        Case::Key(letter)
                    }
                }
            };
        }
    }

    map
}
