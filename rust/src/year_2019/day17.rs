use itertools::Itertools;
use std::collections::HashMap;

use crate::Solution;

type InputType = Vec<i128>;

type Map = Vec<Vec<Block>>;

struct Prg {
    i: i128,
    relative_base: i128,
    instrs: HashMap<usize, i128>,
    inputs: Vec<i128>,
    outputs: Vec<i128>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Robot(Orientation),
    Path,
    Void,
}

#[derive(Clone, Copy)]
enum Mode {
    Immediate,
    Position,
    Relative,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Order {
    Forward(usize),
    R,
    L,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Prg {
    fn eval_i(&self, i: i128, m: Mode) -> i128 {
        match m {
            Mode::Immediate => self.get(i),
            Mode::Position => self.get(self.get(i)),
            Mode::Relative => self.get(self.get(i) + self.relative_base),
        }
    }

    fn get(&self, i: i128) -> i128 {
        *self.instrs.get(&(i as usize)).or(Some(&0)).unwrap()
    }

    fn set(&mut self, i: i128, val: i128, mode: Mode) {
        match mode {
            Mode::Position | Mode::Immediate => {
                let entry = self.instrs.entry(i as usize).or_insert(0);
                *entry = val;
            }
            Mode::Relative => {
                let entry = self
                    .instrs
                    .entry((i + self.relative_base) as usize)
                    .or_insert(0);
                *entry = val;
            }
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    let (h, w) = (map.len(), map[0].len());
    (0..h).for_each(|y| {
        (0..w).for_each(|x| {
            print!(
                "{}",
                match map[y][x] {
                    Block::Robot(o) => match o {
                        Orientation::Up => "^",
                        Orientation::Right => ">",
                        Orientation::Down => "v",
                        Orientation::Left => "<",
                    },
                    Block::Path => "#",
                    Block::Void => ".",
                }
            )
        });
        println!();
    })
}

fn run_intcode(prg: &mut Prg) {
    while prg.i >= 0 && prg.i < prg.instrs.len() as i128 {
        let instr = prg.get(prg.i);
        let code = instr % 100;
        let modes_str = if instr > 100 {
            ((instr - code) / 100).to_string()
        } else {
            "".to_string()
        };
        let mut modes: Vec<Mode> = vec![Mode::Position; 3];
        let args = [prg.get(prg.i + 1), prg.get(prg.i + 2), prg.get(prg.i + 3)];

        for (i, c) in modes_str.chars().rev().enumerate() {
            modes[i] = match c {
                '0' => Mode::Position,
                '1' => Mode::Immediate,
                '2' => Mode::Relative,
                _ => panic!(),
            }
        }

        match code {
            99 => return,
            1 => {
                // add
                prg.set(
                    args[2],
                    prg.eval_i(prg.i + 1, modes[0]) + prg.eval_i(prg.i + 2, modes[1]),
                    modes[2],
                );
                prg.i += 4;
            }
            2 => {
                // mul
                prg.set(
                    args[2],
                    prg.eval_i(prg.i + 1, modes[0]) * prg.eval_i(prg.i + 2, modes[1]),
                    modes[2],
                );
                prg.i += 4;
            }
            3 => {
                // input
                if let Some(x) = prg.inputs.pop() {
                    prg.set(args[0], x, modes[0]);
                } else {
                    return;
                }
                prg.i += 2;
            }
            4 => {
                // output
                prg.outputs.push(prg.eval_i(prg.i + 1, modes[0]));
                prg.i += 2;
            }
            5 => {
                // jump if not 0
                if prg.eval_i(prg.i + 1, modes[0]) != 0 {
                    prg.i = prg.eval_i(prg.i + 2, modes[1])
                } else {
                    prg.i += 3
                }
            }
            6 => {
                // jump if 0
                if prg.eval_i(prg.i + 1, modes[0]) == 0 {
                    prg.i = prg.eval_i(prg.i + 2, modes[1])
                } else {
                    prg.i += 3
                }
            }
            7 => {
                // is less than
                if prg.eval_i(prg.i + 1, modes[0]) < prg.eval_i(prg.i + 2, modes[1]) {
                    prg.set(args[2], 1, modes[2]);
                } else {
                    prg.set(args[2], 0, modes[2]);
                }
                prg.i += 4;
            }
            8 => {
                // is equal
                if prg.eval_i(prg.i + 1, modes[0]) == prg.eval_i(prg.i + 2, modes[1]) {
                    prg.set(args[2], 1, modes[2]);
                } else {
                    prg.set(args[2], 0, modes[2]);
                }
                prg.i += 4;
            }
            // relative base
            9 => {
                prg.relative_base += prg.eval_i(prg.i + 1, modes[0]);
                prg.i += 2;
            }
            _ => panic!("wrong code : {code}"),
        }
    }
}

fn sum_inter(map: &Map) -> usize {
    let (h, w) = (map.len(), map[0].len());
    (0..h)
        .cartesian_product(0..w)
        .map(|(y, x)| match map[y][x] {
            Block::Path => {
                if get_map_neighbours(map, x, y).len() == 4 {
                    x * y
                } else {
                    0
                }
            }
            _ => 0,
        })
        .sum::<usize>()
}

fn map_from_prg(prg: &Prg) -> Map {
    let s = prg.outputs.iter().fold("".to_string(), |acc, i| {
        format!("{}{}", acc, char::from(*i as u8))
    });

    s.split("\n")
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Block::Path,
                    '.' => Block::Void,
                    '^' => Block::Robot(Orientation::Up),
                    '>' => Block::Robot(Orientation::Right),
                    'v' => Block::Robot(Orientation::Down),
                    '<' => Block::Robot(Orientation::Left),
                    _ => panic!(),
                })
                .collect::<Vec<Block>>()
        })
        .filter(|l| l.len() != 0)
        .collect()
}

fn get_map_neighbours(map: &Map, x: usize, y: usize) -> Vec<Orientation> {
    let (h, w) = (map.len(), map[0].len());
    assert!(x < w);
    assert!(y < h);

    let mut neighbours = vec![];

    if y > 0 && map[y - 1][x] == Block::Path {
        neighbours.push(Orientation::Up);
    }

    if x > 0 && map[y][x - 1] == Block::Path {
        neighbours.push(Orientation::Left);
    }

    if y < h - 1 && map[y + 1][x] == Block::Path {
        neighbours.push(Orientation::Down);
    }

    if x < w - 1 && map[y][x + 1] == Block::Path {
        neighbours.push(Orientation::Right);
    }

    neighbours
}

impl Orientation {
    fn next_position(&self, x: usize, y: usize, map: &Map) -> (usize, usize) {
        match *self {
            Orientation::Up => {
                assert!(y > 0);
                (x, y - 1)
            }
            Orientation::Right => {
                assert!(x < map[0].len() - 1);
                (x + 1, y)
            }
            Orientation::Down => {
                assert!(y < map.len() - 1);
                (x, y + 1)
            }
            Orientation::Left => {
                assert!(x > 0);
                (x - 1, y)
            }
        }
    }

    fn opposite(&self) -> Orientation {
        match *self {
            Orientation::Up => Orientation::Down,
            Orientation::Right => Orientation::Left,
            Orientation::Down => Orientation::Up,
            Orientation::Left => Orientation::Right,
        }
    }

    fn goto(&self, o: Orientation) -> Vec<Order> {
        match (*self, o) {
            (Orientation::Up, Orientation::Up) => vec![],
            (Orientation::Up, Orientation::Right) => vec![Order::R],
            (Orientation::Up, Orientation::Down) => vec![Order::R, Order::R],
            (Orientation::Up, Orientation::Left) => vec![Order::L],
            (Orientation::Right, Orientation::Up) => vec![Order::L],
            (Orientation::Right, Orientation::Right) => vec![],
            (Orientation::Right, Orientation::Down) => vec![Order::R],
            (Orientation::Right, Orientation::Left) => vec![Order::R, Order::R],
            (Orientation::Down, Orientation::Up) => vec![Order::R, Order::R],
            (Orientation::Down, Orientation::Right) => vec![Order::L],
            (Orientation::Down, Orientation::Down) => vec![],
            (Orientation::Down, Orientation::Left) => vec![Order::R],
            (Orientation::Left, Orientation::Up) => vec![Order::R],
            (Orientation::Left, Orientation::Right) => vec![Order::R, Order::R],
            (Orientation::Left, Orientation::Down) => vec![Order::L],
            (Orientation::Left, Orientation::Left) => vec![],
        }
    }
}

fn get_orders(map: &Map) -> Vec<Order> {
    let mut orders = vec![];
    let (h, w) = (map.len(), map[0].len());

    let (mut x, mut y) = (0, 0);
    let mut orientation = Orientation::Up;
    let mut found_origin = false;

    for (yi, xi) in (0..h).cartesian_product(0..w) {
        match map[yi][xi] {
            Block::Robot(o) => {
                orientation = o;
                found_origin = true;
                x = xi;
                y = yi;
                break;
            }
            _ => (),
        }
    }

    assert!(found_origin);

    let first_neighbours = get_map_neighbours(map, x, y);
    assert!(
        first_neighbours.len() == 1,
        "first neighbours : {:?}",
        first_neighbours
    );

    let first_orientation = first_neighbours[0];

    // put robot in good direction
    orders.extend(orientation.goto(first_orientation));
    orientation = first_orientation;

    let mut current_line_length = 0;

    loop {
        let comes_from = orientation.opposite();
        let mut neighbours = get_map_neighbours(map, x, y);
        neighbours.retain(|&d| d != comes_from);

        let neighbours_count = neighbours.len();

        if neighbours_count == 3 || (neighbours_count == 1 && orientation == neighbours[0]) {
            (x, y) = orientation.next_position(x, y, map);
        } else if neighbours_count == 1 {
            orders.push(Order::Forward(current_line_length));
            current_line_length = 0;
            orders.extend(orientation.goto(neighbours[0]));
            (x, y) = neighbours[0].next_position(x, y, map);
            orientation = neighbours[0];
        } else if neighbours_count == 0 {
            orders.push(Order::Forward(current_line_length));
            break;
        }
        current_line_length += 1;
    }

    orders
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();

    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs: instrs,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    run_intcode(&mut prg);
    let map = map_from_prg(&prg);

    Solution::from(sum_inter(&map) as i64)
}

/// THIS SOLUTION IS NOT AUTOMATIZED\
/// The path subdivision was made for my personnal solution.\
/// This code prints the path to take though.\
pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let init_instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();

    let mut instrs = init_instrs.clone();
    assert!(instrs[&0usize] == 1);
    instrs.insert(0usize, 2);

    let mut init_prg = Prg {
        i: 0,
        relative_base: 0,
        instrs: init_instrs,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    run_intcode(&mut init_prg);
    let map = map_from_prg(&init_prg);

    #[allow(unused)]
    let orders = get_orders(&map);
    // println!("\n\nOrders to execute : {:?}\n\n", orders);

    let mut solution = String::new();
    // main routine
    solution += "C,B,B,A,A,C,C,B,B,A\n";
    solution += "R,12,R,4,L,6,L,8,L,8\n";
    solution += "R,12,R,4,L,12\n";
    solution += "L,12,R,4,R,4\n";
    solution += "n\n";

    let inputs = solution.chars().map(|c| (c as u32) as i128).rev().collect();

    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs: instrs,
        inputs: inputs,
        outputs: Vec::new(),
    };

    run_intcode(&mut prg);
    let solution = *prg.outputs.last().unwrap();
    // for x in prg.outputs {
    //     print!("{}", char::from_u32(x as u32).unwrap());
    // }

    Solution::from(solution as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>()[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}
