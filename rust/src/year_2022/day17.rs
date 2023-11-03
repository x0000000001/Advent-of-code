use std::collections::{HashMap, HashSet};

use crate::Solution;

type InputType = Vec<bool>;

type Position = (usize, usize);

struct Shape {
    rocks: Vec<Position>,
    width: usize,
    height: usize,
}

fn rock_collides(rocks: &HashSet<Position>, shape: &Shape, (x, y): Position) -> bool {
    for (xp, yp) in shape.rocks.iter() {
        if rocks.contains(&(x + xp, y + yp)) {
            return true;
        }
    }

    return false;
}

#[allow(dead_code)]
fn print_rocks(rocks: &HashSet<Position>, maxy: usize) {
    for j in (0..maxy + 1).rev() {
        for i in 0..7 {
            print!("{}", if rocks.contains(&(i, j)) { "#" } else { "." });
        }
        println!();
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    move_index: usize,
    top_layers: Vec<Vec<bool>>,
}

const LAYERS_COUNT: usize = 20;

/// **memoization** \
/// Keeping track of move_index and the top layers (close to maxy)
/// to determine equivalent situation. \
/// I took arbitrarily the space bar shape (****) as
/// reference shape to compare (comparison between seen situations only happens
/// when space bar is to be dropped).
fn height_of(input: InputType, steps: usize) -> usize {
    let shapes: Vec<Shape> = Vec::from([
        Shape {
            rocks: Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
            width: 4,
            height: 1,
        },
        Shape {
            rocks: Vec::from([(0, 1), (1, 1), (2, 1), (1, 0), (1, 2)]),
            width: 3,
            height: 3,
        },
        Shape {
            rocks: Vec::from([(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)]),
            width: 3,
            height: 3,
        },
        Shape {
            rocks: Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
            width: 1,
            height: 4,
        },
        Shape {
            rocks: Vec::from([(0, 0), (0, 1), (1, 0), (1, 1)]),
            width: 2,
            height: 2,
        },
    ]);

    // index, max y at index
    let mut seen: HashMap<State, (usize, usize)> = HashMap::new();

    let mut rocks: HashSet<Position> =
        HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
    let mut max_y = 0;
    let mut move_index = 0;
    let mut index = 0;

    while index < steps {
        let mut x = 2;
        let mut y = max_y + 4;
        let shape_index = index % 5;
        let shape = &shapes[shape_index % 5];

        if max_y >= LAYERS_COUNT && shape_index == 0 {
            let top_layers: Vec<Vec<bool>> = (0..LAYERS_COUNT)
                .map(|j| {
                    (0..7)
                        .map(|i| rocks.contains(&(i, max_y - LAYERS_COUNT + j + 1)))
                        .collect()
                })
                .collect();

            let state = State {
                move_index,
                top_layers,
            };

            if let Some((former_index, former_max_y)) = seen.get(&state) {
                let steps_over = (steps - index - 1) / (index - former_index);
                if steps_over > 0 {
                    max_y += steps_over * (max_y - former_max_y);
                    index += steps_over * (index - former_index);
                    for j in 0..state.top_layers.len() {
                        for i in 0..state.top_layers[0].len() {
                            rocks.insert((i, max_y - LAYERS_COUNT + j + 1));
                        }
                    }
                    continue;
                }
            }

            seen.insert(state, (index, max_y));
        }

        loop {
            // left / right
            let mut candidate_x = if input[move_index] && (x + shape.width) < 7 {
                x + 1
            } else if !input[move_index] && x > 0 {
                x - 1
            } else {
                x
            };
            let mut candidate_y = y;

            if rock_collides(&rocks, shape, (candidate_x, candidate_y)) {
                candidate_x = x;
            }

            move_index = (move_index + 1) % input.len();

            candidate_y -= 1;

            if rock_collides(&rocks, shape, (candidate_x, candidate_y)) {
                x = candidate_x;
                y = candidate_y + 1;
                break;
            }

            x = candidate_x;
            y = candidate_y;
        }

        max_y = max_y.max(shape.height + y - 1);

        for (xp, yp) in shape.rocks.iter() {
            rocks.insert((x + xp, y + yp));
        }

        index += 1;
        // println!("{index}");
        // print_rocks(&rocks, max_y);
    }

    max_y
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(height_of(input, 2022) as i64)
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    Solution::from(height_of(input, 1000000000000) as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>()[0]
        .chars()
        .map(|c| c == '>')
        .collect()
}
