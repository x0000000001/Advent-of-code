use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub type InputType = Vec<i128>;

struct Prg {
    i: i128,
    relative_base: i128,
    instrs: HashMap<usize, i128>,
    inputs: Vec<i128>,
    outputs: Vec<i128>,
}

#[derive(Clone, Copy)]
enum Mode {
    Immediate,
    Position,
    Relative,
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

// coming from position is put in head, to be tested lastly
fn possible_moves(x: i64, y: i64, oldx: i64, oldy: i64, walls: &HashSet<(i64, i64)>) -> Vec<i128> {
    let mut moves = vec![];

    for (mov, (candx, candy)) in [
        (1, (x, y - 1)),
        (3, (x + 1, y)),
        (2, (x, y + 1)),
        (4, (x - 1, y)),
    ] {
        if candx == oldx && candy == oldy {
            moves.insert(0, mov);
        } else {
            if !walls.contains(&(candx, candy)) {
                moves.push(mov)
            }
        }
    }

    moves
}

fn shortest_path(x: i64, y: i64, walls: &HashSet<(i64, i64)>) -> Option<usize> {
    let (mut minx, maxx, mut miny, maxy) = walls.iter().fold(
        (i64::MAX, 0, i64::MAX, 0),
        |(minx, maxx, miny, maxy), (x, y)| (minx.min(*x), maxx.max(*x), miny.min(*y), maxy.max(*y)),
    );

    if walls.is_empty() {
        minx = 0;
        miny = 0;
    }

    let mut scores = vec![vec![usize::MAX; (maxy - miny + 3) as usize]; (maxx - minx + 3) as usize];
    let mut queue = vec![];
    queue.push(((0, 0), 0));

    while !queue.is_empty() {
        queue.sort_by(|(_, score0), (_, score1)| score1.cmp(score0));
        let ((currentx, currenty), current_score) = queue.pop().unwrap();
        if currentx == x && currenty == y {
            return Some(current_score);
        }

        for candidate_pos in [
            (currentx + 1, currenty),
            (currentx - 1, currenty),
            (currentx, currenty + 1),
            (currentx, currenty - 1),
        ] {
            if walls.contains(&candidate_pos) {
                continue;
            }

            let candidate_score = scores[candidate_pos.0 as usize][candidate_pos.1 as usize];

            if current_score + 1 < candidate_score {
                scores[candidate_pos.0 as usize][candidate_pos.1 as usize] = current_score + 1;
                queue.push((candidate_pos, current_score + 1));
            }
        }
    }

    None
}

fn print_walls(x: i64, y: i64, walls: &HashSet<(i64, i64)>) {
    let (mut minx, maxx, mut miny, maxy) = walls.iter().fold(
        (i64::MAX, 0, i64::MAX, 0),
        |(minx, maxx, miny, maxy), (x, y)| (minx.min(*x), maxx.max(*x), miny.min(*y), maxy.max(*y)),
    );

    if walls.is_empty() {
        minx = 0;
        miny = 0;
    }

    let mut grid = vec![vec![false; (maxy - miny + 3) as usize]; (maxx - minx + 3) as usize];

    for (x, y) in walls {
        grid[(*x - minx + 1) as usize][(*y - miny + 1) as usize] = true;
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!(
                "{}",
                if grid[i][j] {
                    "#"
                } else if (i as i64 - 1 + minx) == x && (j as i64 - 1 + miny) == y {
                    "*"
                } else {
                    "."
                }
            );
        }

        println!();
    }
}

pub fn result_1(input: InputType) -> i64 {
    let instrs = input
        .into_iter()
        .enumerate()
        .collect::<HashMap<usize, i128>>();
    let mut prg = Prg {
        i: 0,
        relative_base: 0,
        instrs,
        inputs: Vec::from([2]),
        outputs: Vec::new(),
    };

    let (mut x, mut y) = (0, 0);
    let mut moves = Vec::from([1, 2, 3, 4]);
    let mut walls: HashSet<(i64, i64)> = HashSet::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    visited.insert((x, y));

    // to follow wall on the right
    let mut orientation = 1;

    loop {
        print_walls(x, y, &walls);
        println!("{x} {y}");
        let (mut newx, mut newy) = (x, y);
        let mov = moves.pop().unwrap();

        match mov {
            1 => newy -= 1,
            3 => newx += 1,
            2 => newy += 1,
            4 => newx -= 1,
            _ => panic!(),
        }

        prg.inputs = Vec::from([mov]);

        run_intcode(&mut prg);
        match prg.outputs.pop().unwrap() {
            0 => {
                walls.insert((newx, newy));
            }
            1 => {
                moves = possible_moves(newx, newy, x, y, &walls);
                x = newx;
                y = newy;
                // if visited.contains(&(newx, newy)) {
                //     walls.insert((newx, newy));
                // } else {
                visited.insert((newx, newy));
                // }
            }
            2 => return shortest_path(x, y, &walls).unwrap() as i64,
            _ => panic!("unknown answer"),
        }
    }
}

pub fn result_2(input: InputType) -> i64 {
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

    input[0].split(",").map(|x| x.parse().unwrap()).collect()
}
