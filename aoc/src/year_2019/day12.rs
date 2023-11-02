use crate::Solution;

extern crate num_integer;

use std::ops::{Add, AddAssign};

type InputType = Vec<P>;

#[derive(Clone, Copy)]
pub struct P {
    x: i64,
    y: i64,
    z: i64,
}

impl AddAssign for P {
    fn add_assign(&mut self, other: P) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Add for P {
    type Output = P;
    fn add(self, other: P) -> P {
        P {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl P {
    fn signs(p0: &P, p1: &P) -> P {
        P {
            x: if p0.x > p1.x {
                -1
            } else if p0.x < p1.x {
                1
            } else {
                0
            },
            y: if p0.y > p1.y {
                -1
            } else if p0.y < p1.y {
                1
            } else {
                0
            },
            z: if p0.z > p1.z {
                -1
            } else if p0.z < p1.z {
                1
            } else {
                0
            },
        }
    }

    fn sum(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

fn update_velocities(pos: &InputType, speeds: &mut InputType) {
    (0..pos.len()).for_each(|i| {
        speeds[i] += pos
            .iter()
            .map(|p| P::signs(&pos[i], p))
            .fold(P { x: 0, y: 0, z: 0 }, |acc, p| acc + p)
    })
}

fn update_positions(pos: &mut InputType, speeds: &InputType) {
    (0..pos.len()).for_each(|i| pos[i] += speeds[i])
}

pub fn part1(s: String) -> Solution {
    let mut pos = parse(s);

    let mut speeds = vec![P { x: 0, y: 0, z: 0 }; pos.len()];

    for _ in 0..1000 {
        update_velocities(&pos, &mut speeds);
        update_positions(&mut pos, &speeds);
    }

    Solution::from(
        (0..pos.len())
            .map(|i| pos[i].sum() * speeds[i].sum())
            .sum::<i64>(),
    )
}

fn update_velocities_1d(pos: &Vec<i64>, speeds: &mut Vec<i64>) {
    (0..pos.len()).for_each(|i| {
        speeds[i] += pos
            .iter()
            .map(|&p| {
                if p > pos[i] {
                    1
                } else if p < pos[i] {
                    -1
                } else {
                    0
                }
            })
            .sum::<i64>()
    })
}

fn update_positions_1d(pos: &mut Vec<i64>, speeds: &Vec<i64>) {
    (0..pos.len()).for_each(|i| pos[i] += speeds[i])
}

fn first_repeating(mut pos: Vec<i64>) -> usize {
    let mut speeds = vec![0; pos.len()];
    let (pos0, speeds0) = (pos.clone(), speeds.clone());

    let mut i = 0;
    while {
        update_velocities_1d(&pos, &mut speeds);
        update_positions_1d(&mut pos, &speeds);
        i += 1;

        pos != pos0 || speeds != speeds0
    } {}

    i
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let xpos = input.iter().map(|p| p.x).collect::<Vec<i64>>();
    let ypos = input.iter().map(|p| p.y).collect::<Vec<i64>>();
    let zpos = input.iter().map(|p| p.z).collect::<Vec<i64>>();

    let (ix, iy, iz) = (
        first_repeating(xpos),
        first_repeating(ypos),
        first_repeating(zpos),
    );

    Solution::from(num_integer::lcm(
        iz as i64,
        num_integer::lcm(ix as i64, iy as i64),
    ))
}

fn read_vec(l: &str) -> P {
    let words = l.split(&[',', '=', '>'][..]).collect::<Vec<&str>>();
    P {
        x: words[1].parse().unwrap(),
        y: words[3].parse().unwrap(),
        z: words[5].parse().unwrap(),
    }
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .map(|l| read_vec(&l))
        .collect()
}
