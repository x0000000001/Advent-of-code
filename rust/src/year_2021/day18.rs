use core::fmt;

use crate::Solution;

type InputType = Vec<Vec<SF>>;

// snailfish
#[derive(Clone, Copy)]
enum SF {
    //brackets
    Left,
    Right,
    //value,
    Val(i64),
}

impl fmt::Debug for SF {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SF::Left => write!(f, "["),
            SF::Right => write!(f, "]"),
            SF::Val(v) => write!(f, "{}", v),
        }
    }
}

//adds n1 to n0
fn sf_add(n0: &mut Vec<SF>, n1: &mut Vec<SF>) {
    n0.insert(0, SF::Left);
    n0.append(n1);
    n0.push(SF::Right);
    sf_reduce(n0);
}

#[allow(dead_code)]
fn sf_print(n: &Vec<SF>) {
    for c in n.iter() {
        match c {
            SF::Left => {
                print!("[")
            }
            SF::Right => {
                print!("]")
            }
            SF::Val(v) => print!("{v} "),
        }
    }

    println!();
}

fn sf_reduce(n: &mut Vec<SF>) {
    // brackets count
    let mut c = 0;

    //exploding
    for i in 0..n.len() {
        match n[i] {
            SF::Left => c += 1,
            SF::Right => c -= 1,
            _ => (),
        }

        if c > 4 {
            if let SF::Val(v_left) = n[i + 1] {
                if let SF::Val(v_right) = n[i + 2] {
                    n.remove(i);
                    n.remove(i);
                    n.remove(i);
                    n.remove(i);

                    //pushing v_left to first number on the left
                    if i > 0 {
                        let mut index = i - 1;
                        loop {
                            match n[index] {
                                SF::Val(v) => {
                                    n[index] = SF::Val(v + v_left);
                                    break;
                                }
                                _ => (),
                            }

                            if index == 0 {
                                break;
                            }

                            index -= 1;
                        }
                    }
                    //pushing v_right to first number on the right
                    for index in i..n.len() {
                        match n[index] {
                            SF::Val(v) => {
                                n[index] = SF::Val(v + v_right);
                                break;
                            }
                            _ => (),
                        }
                    }

                    n.insert(i, SF::Val(0));

                    sf_reduce(n);
                    return;
                }
            }
        }
    }

    c = 0;

    //spliting
    for i in 0..n.len() {
        match n[i] {
            SF::Left => c += 1,
            SF::Right => c -= 1,
            _ => (),
        }

        if let SF::Val(v) = n[i] {
            if v >= 10 {
                // println!("spliting");
                n.remove(i);
                n.insert(i, SF::Left);
                n.insert(i + 1, SF::Val(v / 2));
                n.insert(i + 2, SF::Val((v / 2) + (v % 2)));
                n.insert(i + 3, SF::Right);
                sf_reduce(n);
                return;
            }
        }
    }
}

fn sf_magnitude(n: &[SF]) -> i64 {
    let mut count = 0;
    let mut index = 1;

    let mut right = 0;
    let mut left = 0;

    while index < n.len() {
        match n[index] {
            SF::Left => count += 1,
            SF::Right => count -= 1,
            _ => (),
        }

        if count == 0 {
            if let SF::Val(v) = n[1] {
                left = v;
            } else {
                left = sf_magnitude(&n[1..(index + 1)]);
            }

            if let SF::Val(v) = n[n.len() - 2] {
                right = v;
            } else {
                right = sf_magnitude(&n[(index + 1)..(n.len() - 1)]);
            }

            break;
        }

        index += 1;
    }

    // println!("{:?} : left {}, right {}", n,left, right);

    3 * left + 2 * right
}

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    let mut result = input[0].clone();
    for i in 0..(input.len() - 1) {
        sf_reduce(&mut input[i + 1]);
        sf_add(&mut result, &mut input[i + 1]);
    }

    Solution::from(sf_magnitude(&result))
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);
    let mut result: Vec<SF>;
    let mut max = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            result = input[i].clone();
            sf_add(&mut result, &mut input[j].clone());
            let magnitude = sf_magnitude(&result);

            if magnitude > max {
                max = magnitude;
            }
        }
    }

    Solution::from(max)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut nums: Vec<Vec<SF>> = Vec::new();

    let mut reading_num: bool = false;
    let mut temp_num: Vec<char> = Vec::new();

    for i in 0..input.len() {
        nums.push(vec![]);

        for c in input[i].chars() {
            if c.is_digit(10) {
                if !reading_num {
                    reading_num = true;
                }

                temp_num.push(c);
            } else {
                if reading_num {
                    reading_num = false;
                    nums[i].push(SF::Val(
                        String::from_iter(temp_num.iter()).parse::<i64>().unwrap(),
                    ));
                    temp_num.clear();
                }
            }

            match c {
                '[' => nums[i].push(SF::Left),
                ']' => nums[i].push(SF::Right),
                _ => (),
            }
        }
    }

    nums
}
