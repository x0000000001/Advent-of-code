use crate::Solution;

type InputType = (Vec<(i32, i32)>, Vec<(i32, i32)>);

pub fn part1(s: String) -> Solution {
    let (mut coords, mut folds) = parse(s);

    // println!("{:?}", coords);

    let (sens, m) = folds.pop().unwrap();

    for i in 0..coords.len() {
        match sens {
            0 => {
                if coords[i].0 > m {
                    coords[i].0 = 2 * m - coords[i].0;
                }
            }
            _ => {
                if coords[i].1 > m {
                    coords[i].1 = 2 * m - coords[i].1;
                }
            }
        }
    }

    coords.sort();
    coords.dedup();

    Solution::from(coords.len() as i32)
}

fn get_sheet(coords: &Vec<(i32, i32)>) -> String {
    let height = coords.iter().map(|el| el.1).max().unwrap();
    let width = coords.iter().map(|el| el.0).max().unwrap();

    let mut s = String::new();

    for j in 0..height + 1 {
        for i in 0..width + 1 {
            s += if coords.contains(&(i, j)) { "#" } else { "." };
        }
        s += "\n";
    }

    s
}

pub fn part2(s: String) -> Solution {
    let (mut coords, mut folds) = parse(s);

    while !folds.is_empty() {
        let (sens, m) = folds.pop().unwrap();

        for i in 0..coords.len() {
            match sens {
                0 => {
                    if coords[i].0 > m {
                        coords[i].0 = 2 * m - coords[i].0;
                    }
                }
                _ => {
                    if coords[i].1 > m {
                        coords[i].1 = 2 * m - coords[i].1;
                    }
                }
            }
        }
    }

    coords.sort();
    coords.dedup();

    Solution::from(get_sheet(&coords))
}

fn parse(s: String) -> InputType {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let mut coords: Vec<(i32, i32)> = Vec::new();

    let mut i = 0;

    while !input[i].is_empty() {
        let c = input[i]
            .split(',')
            .map(|el| el.parse().unwrap())
            .collect::<Vec<i32>>();
        coords.push((c[0], c[1]));

        i += 1;
    }

    i += 1;
    let len = input.len();

    //fold : for x, f, ior y
    let mut folds: Vec<(i32, i32)> = Vec::new();

    while i < len {
        folds.insert(
            0,
            (
                {
                    match input[i].chars().collect::<Vec<char>>()[11] {
                        'x' => 0,
                        _ => 1,
                    }
                },
                input[i][13..].parse().unwrap(),
            ),
        );

        i += 1;
    }

    //returned folds are in inversed order, so that it can be poped to get right order
    (coords, folds)
}
