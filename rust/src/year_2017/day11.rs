use crate::Solution;

type InputType = Vec<Dir>;

pub enum Dir {
    N,
    S,
    NE,
    SE,
    NW,
    SW,
}

fn distance_from_origin(mut x: i64, mut y: i64) -> i64 {
    let mut count = 0;

    while x != 0 || y != 0 {
        if x > 0 && y > 0 {
            x -= 1;
            y -= 1;
        } else if x < 0 && y < 0 {
            x += 1;
            y += 1;
        } else if x > 0 {
            x -= 1;
        } else if x < 0 {
            x += 1;
        } else if y > 0 {
            y -= 1;
        } else if y < 0 {
            y += 1;
        }

        count += 1;
    }

    count
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut x = 0;
    let mut y = 0;

    for instr in input {
        match instr {
            Dir::N => y += 1,
            Dir::S => y -= 1,
            Dir::NE => {
                x += 1;
                y += 1
            }
            Dir::SE => x += 1,
            Dir::NW => x -= 1,
            Dir::SW => {
                x -= 1;
                y -= 1
            }
        }
    }

    Solution::from(distance_from_origin(x, y))
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut x = 0;
    let mut y = 0;
    let mut max_from_origin = 0;

    for instr in input {
        match instr {
            Dir::N => y += 1,
            Dir::S => y -= 1,
            Dir::NE => {
                x += 1;
                y += 1
            }
            Dir::SE => x += 1,
            Dir::NW => x -= 1,
            Dir::SW => {
                x -= 1;
                y -= 1
            }
        }

        max_from_origin = distance_from_origin(x, y).max(max_from_origin);
    }

    Solution::from(max_from_origin)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    input[0]
        .split(",")
        .map(|el| match el {
            "n" => Dir::N,
            "s" => Dir::S,
            "ne" => Dir::NE,
            "nw" => Dir::NW,
            "se" => Dir::SE,
            "sw" => Dir::SW,
            _ => panic!(),
        })
        .collect()
}
