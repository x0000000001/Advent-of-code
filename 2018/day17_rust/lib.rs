use std::{collections::HashSet, fs};

pub type InputType = HashSet<(usize, usize)>;

struct Waterflow<'a> {
    walls: &'a HashSet<(usize, usize)>,
    still_water: &'a mut HashSet<(usize, usize)>,
}

fn next_water_position(
    walls: &InputType,
    still_water: &HashSet<(usize, usize)>,
    maxy: usize,
) -> Option<(usize, usize)> {
    let mut current = (500, 0);
    let mut path = HashSet::new();

    loop {
        path.insert(current);

        if current.1 > maxy {
            return None;
        }

        let bottom = (current.0, current.1 + 1);
        if !path.contains(&bottom) && !still_water.contains(&bottom) && !walls.contains(&bottom) {
            current = bottom;
            continue;
        }

        let left = (current.0 - 1, current.1);
        if !path.contains(&left) && !still_water.contains(&left) && !walls.contains(&left) {
            current = left;
            continue;
        }

        let right = (current.0 + 1, current.1);
        if !path.contains(&right) && !still_water.contains(&right) && !walls.contains(&right) {
            current = right;
            continue;
        }

        return Some(current);
    }
}

#[allow(dead_code)]
fn print_walls(walls: &HashSet<(usize, usize)>) {
    let minx = *walls.iter().map(|(x, _)| x).min().unwrap();
    let maxx = *walls.iter().map(|(x, _)| x).max().unwrap();
    let miny = 0;
    let maxy = *walls.iter().map(|(_, y)| y).max().unwrap();

    for j in miny..(maxy + 1) {
        for i in minx..(maxx + 1) {
            print!("{}", if walls.contains(&(i, j)) { "#" } else { "." });
        }

        println!();
    }
}

pub fn result_1(walls: InputType) -> i64 {
    // let mut t = 0;
    // let maxy = *walls.iter().map(|(_, y)| y).max().unwrap();
    // let mut still_water = HashSet::new();

    // loop {
    //     if let Some(p) = next_water_position(&walls, &still_water, maxy) {
    //         println!("{:?}", p);
    //         still_water.insert(p);
    //     } else {
    //         return t;
    //     }

    //     t += 1;
    // }
    0
}

pub fn result_2(walls: InputType) -> i64 {
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

    let mut set = HashSet::new();

    let range_from_str = |l: &str| -> (usize, usize) {
        let w = l.split("..").collect::<Vec<&str>>();
        if w.len() == 1 {
            let x = w[0].parse::<usize>().unwrap();
            (x, x + 1)
        } else {
            (
                w[0].parse::<usize>().unwrap(),
                (w[1].parse::<usize>().unwrap() + 1),
            )
        }
    };

    for l in input {
        let words = l.split_whitespace().collect::<Vec<&str>>();
        let mut range0 = range_from_str(&words[0][2..words[0].len() - 1]);
        let mut range1 = range_from_str(&words[1][2..words[1].len()]);

        match &l[0..1] {
            "y" => {
                let temp = range0;
                range0 = range1;
                range1 = temp;
            }
            _ => (),
        }

        for i in range0.0..range0.1 {
            for j in range1.0..range1.1 {
                set.insert((i, j));
            }
        }
    }

    set
}
