use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::{Index, IndexMut},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Case {
    Elf(usize),
    Goblin(usize),
    Wall,
    Empty,
}

pub type Map = Vec<Vec<Case>>;
pub type InputType = Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Index<&Point> for Map {
    type Output = Case;

    fn index(&self, index: &Point) -> &Self::Output {
        &self[index.y][index.x]
    }
}

impl IndexMut<&Point> for Map {
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        &mut self[index.y][index.x]
    }
}

fn attack_order(map: &Map) -> Vec<Point> {
    (0..map.len())
        .map(move |y| {
            (0..map[0].len())
                .filter(move |x| match map[y][*x] {
                    Case::Goblin(_) | Case::Elf(_) => true,
                    _ => false,
                })
                .map(move |x| Point { x, y })
        })
        .flatten()
        .collect()
}

fn choose_target(map: &Map, ennemies: &Vec<Point>) -> Point {
    let hitpoints = ennemies
        .iter()
        .map(|e| match map[e] {
            Case::Elf(hitpoints) | Case::Goblin(hitpoints) => hitpoints,
            _ => panic!(),
        })
        .enumerate()
        .collect::<Vec<(usize, usize)>>();

    let min_life = hitpoints.iter().map(|&(_, h)| h).min().unwrap();
    let mut candidates = hitpoints
        .into_iter()
        .filter(|&(_, l)| l == min_life)
        .collect::<Vec<(usize, usize)>>();
    candidates.sort_by_key(|&(i, _)| ennemies[i]);

    ennemies[candidates[0].0]
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.y.partial_cmp(&self.y) {
            Some(core::cmp::Ordering::Equal) => (),
            ord => return ord,
        }
        other.x.partial_cmp(&self.x)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn reconstitute_path(from: &HashMap<Point, Point>, origin: Point) -> Vec<Point> {
    let mut p = vec![origin];

    while let Some(&next) = from.get(p.last().unwrap()) {
        p.push(next);
    }

    p
}

fn targets(map: &Map, unit_pos: &Point, target_unit_type: Case) -> Vec<Point> {
    neighbors(map, unit_pos)
        .into_iter()
        .filter(|p| match (map[p], target_unit_type) {
            (Case::Goblin(_), Case::Elf(_)) => true,
            (Case::Elf(_), Case::Goblin(_)) => true,
            _ => false,
        })
        .collect()
}

fn next_move(map: &Map, unit_pos: Point) -> Option<Point> {
    let unit = map[&unit_pos];

    let mut seen = HashSet::new();
    let mut scores = HashMap::new();
    let mut queue = VecDeque::new();
    let mut min = None;
    let mut from: HashMap<Point, Point> = HashMap::new();
    let mut aimed_squares = vec![];

    scores.insert(unit_pos, 0);
    queue.push_back(unit_pos);

    while let Some(point) = queue.pop_front() {
        if seen.contains(&point) {
            continue;
        }

        let score = *scores.get(&point).unwrap();
        let is_in_range = targets(map, &point, unit).len() > 0;
        seen.insert(point);

        if let Some(found_min) = min {
            if score > found_min {
                break;
            } else if !is_in_range {
                continue;
            } else {
                aimed_squares.push(point);
                continue;
            }
        } else if is_in_range {
            aimed_squares.push(point);
            min = Some(score);
            continue;
        }

        for neighbor in neighbors(map, &point)
            .into_iter()
            .filter(|p| map[p] == Case::Empty)
        {
            if seen.contains(&neighbor) {
                continue;
            }

            if let Some(old_score) = scores.get(&neighbor) {
                match old_score.cmp(&(score + 1)) {
                    std::cmp::Ordering::Less => continue,
                    std::cmp::Ordering::Equal => {
                        if let Some(old_from) = from.get(&neighbor) {
                            from.insert(neighbor, *old_from.max(&point));
                        } else {
                            panic!();
                        }
                        continue;
                    }
                    std::cmp::Ordering::Greater => (),
                }
            }

            scores.insert(neighbor, score + 1);
            from.insert(neighbor, point);
            queue.push_back(neighbor);
        }
    }

    aimed_squares.sort();
    if aimed_squares.len() == 0 {
        None
    } else {
        let path = reconstitute_path(&from, *aimed_squares.last().unwrap());
        Some(path[path.len() - 2])
    }
}

fn neighbors(map: &Map, &Point { x, y }: &Point) -> Vec<Point> {
    let mut result = Vec::new();
    if x > 0 {
        result.push(Point { x: x - 1, y });
    }
    if y > 0 {
        result.push(Point { x: x, y: y - 1 });
    }
    if x < map[0].len() - 1 {
        result.push(Point { x: x + 1, y });
    }
    if y < map.len() - 1 {
        result.push(Point { x: x, y: y + 1 });
    }
    result
}

// returns true if kills an ennemy
fn unit_play(
    map: &mut Map,
    mut unit_pos: Point,
    goblins_alive: &mut usize,
    elfs_alive: &mut usize,
    elfs_attack_power: usize,
    moved_cases: &mut HashSet<Point>,
) {
    if moved_cases.contains(&unit_pos) {
        return;
    }

    let unit = map[&unit_pos];

    if map[&unit_pos] == Case::Empty {
        return;
    }

    let mut ts = targets(map, &unit_pos, unit);

    // move
    // todo!("units don't move");
    if ts.len() == 0 {
        if let Some(next_pos) = next_move(map, unit_pos) {
            map[&next_pos] = map[&unit_pos];
            map[&unit_pos] = Case::Empty;
            unit_pos = next_pos;
            ts = targets(map, &unit_pos, unit);
        } else {
            return;
        }
    }

    // attack
    if ts.len() > 0 {
        let target = choose_target(map, &ts);
        map[&target] = match map[&target] {
            Case::Elf(mut hitpoints) => {
                match map[&unit_pos] {
                    Case::Goblin(_) => (),
                    _ => panic!(),
                };
                hitpoints = (hitpoints as i64 - 3).max(0) as usize;
                match hitpoints {
                    0 => {
                        *elfs_alive -= 1;
                        Case::Empty
                    }
                    x => Case::Elf(x),
                }
            }
            Case::Goblin(mut hitpoints) => {
                match map[&unit_pos] {
                    Case::Elf(_) => (),
                    _ => panic!(),
                };
                hitpoints = (hitpoints as i64 - elfs_attack_power as i64).max(0) as usize;
                match hitpoints {
                    0 => {
                        *goblins_alive -= 1;
                        Case::Empty
                    }
                    x => Case::Goblin(x),
                }
            }
            _ => panic!(),
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for y in 0..map.len() {
        let mut units_on_line = vec![];
        for x in 0..map[0].len() {
            print!(
                "{}",
                match map[y][x] {
                    elf @ Case::Elf(_) => {
                        units_on_line.push(elf);
                        'E'
                    }
                    goblin @ Case::Goblin(_) => {
                        units_on_line.push(goblin);
                        'G'
                    }
                    Case::Wall => '#',
                    Case::Empty => '.',
                }
            )
        }
        print!("   ");

        for unit in units_on_line {
            print!(
                "{}, ",
                match unit {
                    Case::Elf(x) => format!("{}({})", "E", x),
                    Case::Goblin(x) => format!("{}({})", "G", x),
                    _ => panic!(),
                }
            );
        }

        println!();
    }
}

/// Returns (have elfs won, map score)
fn play(mut map: Map, elfs_attack_power: usize) -> (bool, usize) {
    let mut globlins_alive = 0;
    let mut elfs_alives = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                Case::Elf(_) => elfs_alives += 1,
                Case::Goblin(_) => globlins_alive += 1,
                _ => (),
            }
        }
    }
    let mut finished = false;
    let mut full_rounds_completed = 0;

    while !finished {
        // print_map(&map);
        let mut moved_cases = HashSet::new();

        for unit_pos in attack_order(&map) {
            if globlins_alive == 0 || elfs_alives == 0 {
                finished = true;
                break;
            }

            unit_play(
                &mut map,
                unit_pos,
                &mut globlins_alive,
                &mut elfs_alives,
                elfs_attack_power,
                &mut moved_cases,
            );
        }

        if finished {
            break;
        } else {
            full_rounds_completed += 1;
        }
    }

    let sum_of_hipoints = (0..map.len())
        .map(|y| {
            (0..map[0].len())
                .map(|x| match map[y][x] {
                    Case::Goblin(hitpoints) | Case::Elf(hitpoints) => hitpoints,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    // println!(
    //     "sum of hitpoints : {} full rounds completed {}",
    //     sum_of_hipoints, full_rounds_completed
    // );

    (globlins_alive == 0, sum_of_hipoints * full_rounds_completed)
}

pub fn result_1(map: Map) -> i64 {
    play(map, 3).1 as i64
}

// 27089 too low
pub fn result_2(map: Map) -> i64 {
    let mut i = 4;

    loop {
        let (have_elfs_won, score) = play(map.clone(), i);

        if have_elfs_won {
            return score as i64;
        }

        i += 1;
    }
}

pub fn read_input(path: &str) -> Map {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| match c {
                    'E' => Case::Elf(200),
                    'G' => Case::Goblin(200),
                    '#' => Case::Wall,
                    '.' => Case::Empty,
                    _ => panic!("Unknown case"),
                })
                .collect()
        })
        .collect()
}
