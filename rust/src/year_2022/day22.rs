use std::collections::HashMap;

use crate::Solution;

type InputType = (Vec<Vec<Tile>>, Vec<Instruction>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Free,
    Nothing,
}

pub enum Instruction {
    Right,
    Left,
    Forward(usize),
}

fn next(mut x: usize, mut y: usize, facing: usize, map: &Vec<Vec<Tile>>) -> (bool, (usize, usize)) {
    let (h, w) = (map.len(), map[0].len());
    loop {
        (x, y) = match facing {
            0 => ((x + h - 1) % h, y),
            1 => (x, (y + 1) % w),
            2 => ((x + 1) % h, y),
            3 => (x, (y + w - 1) % w),
            _ => panic!(),
        };

        match map[x][y] {
            Tile::Free => return (true, (x, y)),
            Tile::Nothing => (),
            Tile::Wall => return (false, (0, 0)),
        }
    }
}

pub fn part1(s: String) -> Solution {
    let (map, instructions) = parse(s);

    let (mut x, mut y) = (0, 0);
    let mut facing = 1;

    while map[x][y] != Tile::Free {
        y += 1;
    }

    for instr in instructions {
        match instr {
            Instruction::Right => facing = (facing + 1) % 4,
            Instruction::Left => facing = (facing + 3) % 4,
            Instruction::Forward(mut n) => {
                while n > 0 {
                    let (can_move, (candx, candy)) = next(x, y, facing, &map);
                    if !can_move {
                        break;
                    }

                    n -= 1;
                    (x, y) = (candx, candy);
                }
            }
        }
    }

    Solution::from(
        ((x + 1) * 1000
            + (y + 1) * 4
            + match facing {
                0 => 3,
                1 => 0,
                2 => 1,
                3 => 2,
                _ => panic!(),
            }) as i64,
    )
}

type Position = ((usize, usize), usize);
type Transitions = HashMap<Position, Position>;

fn next2(
    x: usize,
    y: usize,
    facing: usize,
    map: &Vec<Vec<Tile>>,
    transitions: &Transitions,
) -> Option<Position> {
    let (h, w) = (map.len(), map[0].len());
    if let Some(&((newx, newy), new_facing)) = transitions.get(&((x, y), facing)) {
        match map[newx][newy] {
            Tile::Wall => return None,
            Tile::Free => return Some(((newx, newy), new_facing)),
            Tile::Nothing => panic!(),
        }
    }

    let (candx, candy) = match facing {
        0 => ((x + h - 1) % h, y),
        1 => (x, (y + 1) % w),
        2 => ((x + 1) % h, y),
        3 => (x, (y + w - 1) % w),
        _ => panic!(),
    };

    match map[candx][candy] {
        Tile::Free => Some(((candx, candy), facing)),
        Tile::Nothing => panic!(),
        Tile::Wall => None,
    }
}

fn get_transitions(map: &Vec<Vec<Tile>>) -> HashMap<Position, Position> {
    let (h, w) = (map.len(), map[0].len());
    let n: usize = map
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| match c {
                    Tile::Wall => 1,
                    Tile::Free => 1,
                    Tile::Nothing => 0,
                })
                .sum()
        })
        .min()
        .unwrap();

    let width = w / n;
    let height = h / n;
    let scheme: Vec<Vec<bool>> = (0..height)
        .map(|i| {
            (0..width)
                .map(|j| match map[i * n][j * n] {
                    Tile::Nothing => false,
                    _ => true,
                })
                .collect()
        })
        .collect();

    let faces_to_glue: Vec<Vec<Vec<usize>>> = (0..height)
        .map(|i| {
            (0..width)
                .map(|j| {
                    if scheme[i][j] {
                        let mut v = vec![];

                        if i == 0 || !scheme[i - 1][j] {
                            v.push(0);
                        }

                        if j == width - 1 || !scheme[i][j + 1] {
                            v.push(1);
                        }

                        if i == height - 1 || !scheme[i + 1][j] {
                            v.push(2);
                        }

                        if j == 0 || !scheme[i][j - 1] {
                            v.push(3);
                        }

                        v
                    } else {
                        vec![]
                    }
                })
                .collect()
        })
        .collect();

    let mut transitions = HashMap::new();
    let mut links = HashMap::new();

    // NON GENERAL SOLUTION PART

    // TEST INPUT
    // links.insert(((0, 2), 0), ((1, 0), 0));
    // links.insert(((1, 0), 0), ((0, 2), 0));
    // links.insert(((0, 2), 3), ((1, 1), 0));
    // links.insert(((1, 1), 0), ((0, 2), 3));
    // links.insert(((0, 2), 1), ((2, 3), 1));
    // links.insert(((2, 3), 1), ((0, 2), 1));

    // links.insert(((1, 0), 2), ((2, 2), 2));
    // links.insert(((2, 2), 2), ((1, 0), 2));
    // links.insert(((1, 0), 3), ((2, 3), 2));
    // links.insert(((2, 3), 2), ((1, 0), 3));

    // links.insert(((1, 1), 2), ((2, 2), 3));
    // links.insert(((2, 2), 3), ((1, 1), 2));

    // links.insert(((1, 2), 1), ((2, 3), 0));
    // links.insert(((2, 3), 0), ((1, 2), 1));
    //

    // INPUT
    links.insert(((0, 1), 0), ((3, 0), 3));
    links.insert(((3, 0), 3), ((0, 1), 0));
    links.insert(((0, 1), 3), ((2, 0), 3));
    links.insert(((2, 0), 3), ((0, 1), 3));

    links.insert(((0, 2), 0), ((3, 0), 2));
    links.insert(((3, 0), 2), ((0, 2), 0));
    links.insert(((0, 2), 1), ((2, 1), 1));
    links.insert(((2, 1), 1), ((0, 2), 1));
    links.insert(((0, 2), 2), ((1, 1), 1));
    links.insert(((1, 1), 1), ((0, 2), 2));

    links.insert(((1, 1), 3), ((2, 0), 0));
    links.insert(((2, 0), 0), ((1, 1), 3));

    links.insert(((2, 1), 2), ((3, 0), 1));
    links.insert(((3, 0), 1), ((2, 1), 2));
    //

    //

    for i in 0..height {
        for j in 0..width {
            for &facing in faces_to_glue[i][j].iter() {
                let ((jumpx, jumpy), jump_facing) = *links.get(&((i, j), facing)).unwrap();
                let player_facing = (jump_facing + 2) % 4;

                for k in 0..n {
                    transitions.insert(
                        (get_global_position(((i, j), facing), k, n), facing),
                        (
                            get_global_position(((jumpx, jumpy), jump_facing), n - k - 1, n),
                            player_facing,
                        ),
                    );
                }
            }
        }
    }

    transitions
}

fn get_global_position(((x, y), facing): Position, k: usize, n: usize) -> (usize, usize) {
    match facing {
        0 => (x * n, y * n + k),
        1 => (x * n + k, (y + 1) * n - 1),
        2 => ((x + 1) * n - 1, y * n + n - k - 1),
        3 => (x * n + n - k - 1, y * n),
        _ => panic!(),
    }
}

pub fn part2(s: String) -> Solution {
    let (map, instructions) = parse(s);

    let (mut x, mut y) = (0, 0);
    let mut facing = 1;

    while map[x][y] != Tile::Free {
        y += 1;
    }

    let transitions = get_transitions(&map);

    for instr in instructions {
        match instr {
            Instruction::Right => facing = (facing + 1) % 4,
            Instruction::Left => facing = (facing + 3) % 4,
            Instruction::Forward(mut n) => {
                while n > 0 {
                    if let Some(((newx, newy), new_facing)) =
                        next2(x, y, facing, &map, &transitions)
                    {
                        n -= 1;
                        (x, y) = (newx, newy);
                        facing = new_facing;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    Solution::from(
        ((x + 1) * 1000
            + (y + 1) * 4
            + match facing {
                0 => 3,
                1 => 0,
                2 => 1,
                3 => 2,
                _ => panic!(),
            }) as i64,
    )
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let h = input.len() - 1;
    let w = (0..h).map(|i| input[i].len()).max().unwrap();
    let mut map = vec![vec![Tile::Nothing; w]; h];

    for i in 0..h {
        for (j, c) in input[i].chars().enumerate() {
            map[i][j] = match c {
                '.' => Tile::Free,
                '#' => Tile::Wall,
                ' ' => Tile::Nothing,
                _ => panic!(),
            };
        }
    }

    let mut instrs = vec![];

    let mut i = 0;
    let chars = input.last().unwrap().chars().collect::<Vec<char>>();

    while i < chars.len() {
        let c = chars[i];
        match c {
            'R' => instrs.push(Instruction::Right),
            'L' => instrs.push(Instruction::Left),
            _ => {
                let mut w = vec![];
                let mut j = i;
                while j < chars.len() && chars[j].is_digit(10) {
                    w.push(chars[j]);
                    j += 1;
                }
                instrs.push(Instruction::Forward(
                    w.iter().collect::<String>().parse().unwrap(),
                ));
                i = j - 1;
            }
        }

        i += 1;
    }

    (map, instrs)
}
