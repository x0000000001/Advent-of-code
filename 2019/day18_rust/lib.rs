use std::{collections::HashMap, fs};

pub type Map = Vec<Vec<bool>>;
type Position = (usize, usize);
type ObjectSet = usize;

pub type InputType = (Map, Position, HashMap<Position, usize>);

fn set_add(set: ObjectSet, object: usize) -> ObjectSet {
    set | (1 << object)
}

fn set_remove(set: ObjectSet, object: usize) -> ObjectSet {
    set & !(1 << object)
}

fn set_contains(set: ObjectSet, object: usize) -> bool {
    (set >> object) % 2 == 1
}

fn set_to_vec(set: ObjectSet) -> Vec<usize> {
    (0..64)
        .filter(|i| (set >> i) % 2 == 0)
        .into_iter()
        .collect()
}

// returns, for each item in items, the distance from start
// and the doors to open to get to
fn paths_from_start(
    map: &Map,
    start: Position,
    items: &HashMap<Position, usize>,
) -> HashMap<usize, (usize, ObjectSet)> {
    let (h, w) = (map.len(), map[0].len());
    let mut distances = vec![vec![usize::MAX; w]; h];
    let mut obstacles = vec![vec![0; w]; h];
    let mut queue = vec![];
    queue.push((0, start));
    distances[start.0][start.1] = 0;

    while !queue.is_empty() {
        queue.sort_by_key(|(score, _)| usize::MAX - score);
        let (score, position) = queue.pop().unwrap();

        todo!();

        let mut candidates = vec![];
    }

    items
        .iter()
        .map(|(p, code)| (*code, (distances[p.0][p.1], obstacles[p.0][p.1])))
        .collect()
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct GameState {
    keys_found: ObjectSet,
    position: Position,
    moves_count: usize,
    objects_remaining: ObjectSet,
}

impl GameState {
    fn print(&self) {
        println!("keys found : {:#b}", self.keys_found);
        println!("position : {:?}", self.position);
        println!("moves count : {:?}", self.moves_count);
        println!("remaining objects : {:#b}", self.objects_remaining);
        println!();
    }
}

pub fn result_1((map, pos, objects): InputType) -> i64 {
    0
}

pub fn result_2((map, position, objects): InputType) -> i64 {
    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let (h, w) = (input.len(), input[0].len());
    let mut map = vec![vec![false; w]; h];
    let mut player_position = (0, 0);
    let mut objects = HashMap::new();

    for i in 0..h {
        let mut chars = input[i].chars();
        for j in 0..w {
            let c = chars.next().unwrap();
            map[i][j] = match c {
                '#' => true,
                '.' => false,
                '@' => {
                    player_position = (i, j);
                    false
                }
                _ => {
                    objects.insert(
                        (i, j),
                        if c.is_lowercase() {
                            (c as u32 - 'a' as u32) as usize
                        } else {
                            (c as u32 - 'A' as u32) as usize + 25
                        },
                    );
                    false
                }
            };
        }
    }

    (map, player_position, objects)
}

// // (item, distance, position), sorted by distance
// fn accessible_items(
//     map: &Map,
//     pos: (usize, usize),
//     objects: &ObjectList,
// ) -> Vec<(char, usize, (usize, usize))> {
//     let (h, w) = (map.len(), map[0].len());
//     let mut scores = HashMap::new();
//     scores.insert(pos, 0);
//     let mut queue = vec![];
//     queue.push((0, pos));
//     let mut accessibles = vec![];

//     while !queue.is_empty() {
//         queue.sort_by_key(|(score, _)| w * h - score);
//         let (score, (x, y)) = queue.pop().unwrap();
//         let mut candidates = vec![];

//         if x > 0 && !scores.contains_key(&(x - 1, y)) && !map[x - 1][y] {
//             candidates.push((x - 1, y));
//         }
//         if y > 0 && !scores.contains_key(&(x, y - 1)) && !map[x][y - 1] {
//             candidates.push((x, y - 1));
//         }
//         if x < h - 1 && !scores.contains_key(&(x + 1, y)) && !map[x + 1][y] {
//             candidates.push((x + 1, y));
//         }
//         if y < w - 1 && !scores.contains_key(&(x, y + 1)) && !map[x][y + 1] {
//             candidates.push((x, y + 1));
//         }

//         let new_score = score + 1;

//         for (candidatex, candidatey) in candidates {
//             // doors can be accessed but we can't go through
//             let mut is_door = false;
//             for i in 0..objects.len() {
//                 if objects[i].0 .0 == candidatex && objects[i].0 .1 == candidatey {
//                     accessibles.push((objects[i].1, new_score, objects[i].0));
//                     if objects[i].1.is_uppercase() {
//                         is_door = true;
//                     }
//                     break;
//                 }
//             }
//             if !is_door {
//                 queue.push((new_score, (candidatex, candidatey)));
//             }

//             scores.insert((candidatex, candidatey), new_score);
//         }
//     }

//     accessibles
// }

// pub fn result_1((map, pos, objects): InputType) -> i64 {
//     let mut queue: Vec<GameState> = vec![];
//     let mut scores: HashSet<GameState> = HashSet::new();

//     let init_game_state = GameState {
//         keys_found: vec![],
//         objects_remaining: objects,
//         position: pos,
//         moves_count: 0,
//     };
//     queue.push(init_game_state.clone());
//     scores.insert(init_game_state);

//     while !queue.is_empty() {
//         queue.sort_by_key(|game_state| usize::MAX - game_state.moves_count);
//         let game_state = queue.pop().unwrap();

//         // game_state.print();

//         if game_state.objects_remaining.len() == 0 {
//             return game_state.moves_count as i64;
//         }

//         let accessibles =
//             accessible_items(&map, game_state.position, &game_state.objects_remaining);

//         for (item, distance, item_position) in accessibles {
//             // doors can't be accessed without their corresponding keys
//             if item.is_uppercase()
//                 && !game_state
//                     .keys_found
//                     .contains(&item.to_lowercase().next().unwrap())
//             {
//                 continue;
//             }

//             let mut new_keys = game_state.keys_found.clone();
//             if item.is_lowercase() {
//                 new_keys.push(item);
//             }

//             let mut new_objects = game_state.objects_remaining.clone();
//             let remove_index = new_objects
//                 .iter()
//                 .enumerate()
//                 .filter(|(_, (_, c))| *c == item)
//                 .map(|(i, _)| i)
//                 .next()
//                 .unwrap();
//             new_objects.remove(remove_index);

//             let new_game_state = GameState {
//                 keys_found: new_keys,
//                 objects_remaining: new_objects,
//                 position: item_position,
//                 moves_count: game_state.moves_count + distance,
//             };

//             if scores.contains(&new_game_state) {
//                 continue;
//             }

//             scores.insert(new_game_state.clone());
//             queue.push(new_game_state);
//         }
//     }

//     -1
// }
