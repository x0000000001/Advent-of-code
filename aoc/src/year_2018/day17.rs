use std::collections::HashSet;

use crate::Solution;

type InputType = HashSet<(usize, usize)>;

#[allow(dead_code)]
fn print_state(walls: &HashSet<(usize, usize)>, water: &HashSet<(usize, usize)>) {
    let minx = *walls.iter().map(|(x, _)| x).min().unwrap();
    let maxx = *walls.iter().map(|(x, _)| x).max().unwrap();
    let miny = 0;
    let maxy = *walls.iter().map(|(_, y)| y).max().unwrap();

    for j in miny..(maxy + 1) {
        for i in minx..(maxx + 1) {
            print!(
                "{}",
                if water.contains(&(i, j)) {
                    "~"
                } else if walls.contains(&(i, j)) {
                    "#"
                } else {
                    "."
                }
            );
        }

        println!();
    }
}

// returns (total water (part 1), water that won't drain (part2))
fn compute_water(mut walls: InputType) -> (usize, usize) {
    let maxy = *walls.iter().map(|(_, y)| y).max().unwrap();
    let miny = *walls.iter().map(|(_, y)| y).min().unwrap();

    let original_walls_count = walls.len();

    let mut water: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = vec![];
    queue.push((500, 0));
    water.insert((500, 0));

    while let Some(node @ (x, y)) = queue.pop() {
        // print_state(&walls, Some(&water));

        water.insert(node);

        if y == maxy {
            continue;
        }

        let bottom = (x, y + 1);

        if !walls.contains(&bottom) {
            if water.contains(&bottom) {
                continue;
            }

            queue.push(node);
            queue.push(bottom);
            continue;
        }

        let is_blocked_left;
        let mut xleft = x;

        loop {
            if walls.contains(&(xleft - 1, y)) {
                is_blocked_left = true;
                break;
            }

            if !walls.contains(&(xleft, y + 1)) {
                is_blocked_left = false;
                break;
            }

            xleft -= 1;
        }

        let is_blocked_right;
        let mut xright = x;

        loop {
            if walls.contains(&(xright + 1, y)) {
                is_blocked_right = true;
                break;
            }

            if !walls.contains(&(xright, y + 1)) {
                is_blocked_right = false;
                break;
            }

            xright += 1;
        }

        if is_blocked_left && is_blocked_right {
            walls.insert(node);
            // wont_drain_count += 1;

            for i in (x + 1)..(xright + 1) {
                walls.insert((i, y));
                // wont_drain_count += 1;
            }

            for i in xleft..x {
                walls.insert((i, y));
                // wont_drain_count += 1;
            }
        }

        for i in (x + 1)..(xright + 1) {
            water.insert((i, y));
        }

        for i in xleft..x {
            water.insert((i, y));
        }

        if !is_blocked_left {
            queue.push((xleft, y));
        }

        if !is_blocked_right {
            queue.push((xright, y));
        }
    }

    // print_state(&walls, &water);

    (
        water
            .iter()
            .filter(|&(_, y)| *y >= miny && *y <= maxy)
            .count(),
        walls.len() - original_walls_count,
    )
}

pub fn part1(s: String) -> Solution {
    let walls = parse(s);

    Solution::from(compute_water(walls).0 as i64)
}

pub fn part2(s: String) -> Solution {
    let walls = parse(s);

    Solution::from(compute_water(walls).1 as i64)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
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
            "x" => (),
            _ => panic!(),
        }

        for i in range0.0..range0.1 {
            for j in range1.0..range1.1 {
                set.insert((i, j));
            }
        }
    }

    set
}
