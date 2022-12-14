use std::{collections::HashSet, fs};

pub type InputType = HashSet<(usize, usize)>;

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
    let maxy = *walls.iter().map(|(_, y)| y).max().unwrap();
    let miny = *walls.iter().map(|(_, y)| y).min().unwrap();
    let mut still_water: HashSet<(usize, usize)> = HashSet::new();
    let mut moving_water: HashSet<(usize, usize)> = HashSet::new();
    let mut current_path = vec![];
    current_path.push((500, 0));

    while let Some((x, y)) = current_path.pop() {
        moving_water.insert((x, y));

        let bottom = (x, y + 1);
        let bottom_in_still = still_water.contains(&bottom);
        let bottom_in_moving = moving_water.contains(&bottom);
        let bottom_in_walls = walls.contains(&bottom);

        // de-stacks
        if bottom_in_moving && !bottom_in_still {
            current_path.pop();
            continue;
        }

        // stacks down
        if !bottom_in_walls && !bottom_in_still {
            current_path.push(bottom);
            continue;
        }

        let left = (x - 1, y);
        let left_in_still = still_water.contains(&left);
        let left_in_moving = moving_water.contains(&left);
        let left_in_walls = walls.contains(&left);

        let right = (x + 1, y);
        let right_in_still = still_water.contains(&right);
        let right_in_moving = moving_water.contains(&right);
        let right_in_walls = walls.contains(&right);

        // still_water
        if (right_in_still || right_in_walls)
    }

    still_water
        .union(&moving_water)
        .collect::<Vec<&(usize, usize)>>()
        .len() as i64
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

// loop {
//         let mut waters = vec![];
//         waters.push((500, 0));
//         let mut all_waters_dropped = true;
//         let mut seen = HashSet::new();

//         while let Some((x, y)) = waters.pop() {
//             if y > maxy {
//                 continue;
//             }

//             moving_water.insert((x, y));
//             seen.insert((x, y));

//             let bottom = (x, y + 1);
//             let left = (x + 1, y);
//             let right = (x - 1, y);
//             let is_bottom_free =
//                 !still_water.contains(&bottom) && !walls.contains(&bottom) && !seen.contains(&left);
//             let is_left_free =
//                 !still_water.contains(&left) && !walls.contains(&left) && !seen.contains(&left);
//             let is_right_free =
//                 !still_water.contains(&right) && !walls.contains(&right) && !seen.contains(&left);

//             if is_bottom_free {
//                 waters.push(bottom);
//                 continue;
//             }

//             if !is_left_free && !is_right_free {
//                 still_water.insert((x, y));
//                 all_waters_dropped = false;
//             }

//             if is_left_free {
//                 waters.push(left);
//             }

//             if is_right_free {
//                 waters.push(right);
//             }
//         }

//         if all_waters_dropped {
//             break;
//         }
//     }
