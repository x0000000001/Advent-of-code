use crate::Solution;
use std::collections::HashSet;

type InputType = Vec<Vec<Direction>>;

pub enum Direction {
    E,
    Se,
    Sw,
    W,
    Nw,
    Ne,
}

type Board = HashSet<(i64, i64)>;

fn get_dirs(l: &str) -> Vec<Direction> {
    let mut d = Vec::new();
    let mut chars = l.chars().peekable();

    loop {
        if chars.peek() == None {
            break;
        }

        match chars.next().unwrap() {
            'e' => d.push(Direction::E),
            'w' => d.push(Direction::W),
            'n' => match chars.next().unwrap() {
                'e' => d.push(Direction::Ne),
                'w' => d.push(Direction::Nw),
                _ => panic!(),
            },
            's' => match chars.next().unwrap() {
                'e' => d.push(Direction::Se),
                'w' => d.push(Direction::Sw),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    d
}

fn get_init(instrs: InputType) -> Board {
    let mut flipped: Board = HashSet::new();

    for dirs in instrs.iter() {
        let (mut x, mut y) = (0i64, 0i64);

        for d in dirs {
            match d {
                Direction::E => x += 1,
                Direction::Se => {
                    y += 1;
                    x += 1;
                }
                Direction::Sw => y += 1,
                Direction::W => x -= 1,
                Direction::Nw => {
                    y -= 1;
                    x -= 1;
                }
                Direction::Ne => y -= 1,
            }
        }

        if flipped.contains(&(x, y)) {
            flipped.remove(&(x, y));
        } else {
            flipped.insert((x, y));
        }
    }

    flipped
}

pub fn part1(s: String) -> Solution {
    let instrs = parse(s);

    Solution::from(get_init(instrs).len() as i64)
}

fn iter_board(b: Board) -> Board {
    let mut newb = HashSet::new();
    let (minx, maxx, miny, maxy) = b.iter().fold(
        (i64::MAX, 0, i64::MAX, 0),
        |(minx, maxx, miny, maxy), &(x, y)| (minx.min(x), maxx.max(x), miny.min(y), maxy.max(y)),
    );

    for i in (minx - 2)..(maxx + 2) {
        for j in (miny - 2)..(maxy + 2) {
            let neighbours: usize = [
                (i - 1, j),
                (i + 1, j),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j + 1),
                (i - 1, j - 1),
            ]
            .map(|(x, y)| if b.contains(&(x, y)) { 1 } else { 0 })
            .iter()
            .sum();

            if b.contains(&(i, j)) {
                if !(neighbours == 0 || neighbours > 2) {
                    newb.insert((i, j));
                }
            } else {
                if neighbours == 2 {
                    newb.insert((i, j));
                }
            }
        }
    }

    newb
}

pub fn part2(s: String) -> Solution {
    let instrs = parse(s);

    let mut board = get_init(instrs);

    for _ in 0..100 {
        board = iter_board(board);
    }

    Solution::from(board.len() as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .map(|l| get_dirs(&l))
        .collect()
}
