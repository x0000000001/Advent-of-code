use std::{collections::HashMap, fs};

pub type InputType = Vec<Blueprint>;

// ressources in order :
// ore, clay, obsidian

pub type Ores = Vec<usize>;
pub type Robots = Vec<usize>;
pub type Time = usize;

pub struct Blueprint {
    id: usize,
    costs: Vec<Ores>,
}

/// score is maximum number of geodes crafted
fn blueprint_score(
    bp: &Blueprint,
    seen: &mut HashMap<(Time, Ores, Robots), usize>,
    (time, ores, robots): &(Time, Ores, Robots),
    global_max: &mut usize,
) -> usize {
    assert_eq!(ores.len(), 4);
    assert_eq!(robots.len(), 4);

    if *time == 0 {
        return ores[3];
    }

    let mut max = 0;

    let mut neighbors = vec![];

    for i in (0..4).rev() {
        // optimization 2
        // three first ressources are used only to build other robots
        // it is therefore useless to have more robots of one of this type
        // than the maximum ressources needed to build other robots
        // (since we can produce only 1 robot per round)
        // 60s -> 3s
        if i != 3 {
            let has_enough = (0..4)
                .map(|robot_id| robot_id == i || bp.costs[robot_id][i] <= robots[i])
                .reduce(|acc, b| acc && b)
                .unwrap();

            if has_enough {
                continue;
            }
        }
        //

        let waiting_time = (0..4)
            .map(|j| {
                if bp.costs[i][j] <= ores[j] {
                    0
                } else {
                    if robots[j] != 0 {
                        let needed = bp.costs[i][j] - ores[j];
                        needed / robots[j] + (needed % robots[j] != 0) as usize
                    } else {
                        usize::MAX - 2
                    }
                }
            })
            .max()
            .unwrap();

        if waiting_time >= *time {
            continue;
        }

        let n_ores: Ores = ores
            .iter()
            .enumerate()
            .map(|(j, &o)| o + (waiting_time + 1) * robots[j] - bp.costs[i][j])
            .collect();

        let mut n_robots = robots.clone();
        n_robots[i] += 1;

        neighbors.push((*time - (waiting_time + 1), n_ores, n_robots));

        // optimization 1
        // if we can build a geode robot without waiting, useless to explore other paths
        // than building a geode robot (since we can produce only 1 per round)
        // 300s -> 60s
        if i == 3 && waiting_time == 0 {
            continue;
        }
    }

    if neighbors.is_empty() {
        neighbors.push((
            0,
            ores.iter()
                .enumerate()
                .map(|(i, &o)| o + time * robots[i])
                .collect(),
            robots.clone(),
        ))
    }

    // print!("");
    for n in neighbors {
        let max_geodes = n.1[3] + n.0 * n.2[3] + if n.0 > 1 { (n.0 - 2) * (n.0 - 1) } else { 0 };
        if max_geodes < *global_max {
            continue;
        }

        let n_s;
        if let Some(&s) = seen.get(&n) {
            n_s = s;
        } else {
            n_s = blueprint_score(bp, seen, &n, global_max);
            seen.insert(n, n_s);
        }
        max = max.max(n_s);
    }

    if max > *global_max {
        *global_max = max;
    }

    max
}

// 944 too low
pub fn result_1(input: InputType) -> i64 {
    input
        .into_iter()
        .map(|bp| {
            let score = blueprint_score(
                &bp,
                &mut HashMap::new(),
                &(24, vec![0; 4], Vec::from([1, 0, 0, 0])),
                &mut 0,
            );
            println!("id {} score {}", bp.id, score);
            bp.id * score
        })
        .sum::<usize>() as i64
}

pub fn result_2(input: InputType) -> i64 {
    input
        .into_iter()
        .take(3)
        .map(|bp| {
            let score = blueprint_score(
                &bp,
                &mut HashMap::new(),
                &(32, vec![0; 4], Vec::from([1, 0, 0, 0])),
                &mut 0,
            );
            println!("id {} score {}", bp.id, score);
            score
        })
        .product::<usize>() as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let get_ressource_list = |ws: &Vec<&str>| -> Vec<usize> {
        let mut v = vec![0; 4];

        let mut i = 4;

        while i < ws.len() {
            if ws[i] == "and" {
                i += 1;
            }

            let value = ws[i].parse().unwrap();
            match ws[i + 1] {
                "ore" => v[0] = value,
                "clay" => v[1] = value,
                "obsidian" => v[2] = value,
                _ => panic!(),
            }

            i += 2;
        }

        v
    };

    let get_bp = |l: &str| -> Blueprint {
        let words = l
            .split(&['.', ':'][..])
            .map(|l| l.split_whitespace().collect())
            .collect::<Vec<Vec<&str>>>();

        Blueprint {
            id: words[0][1].parse().unwrap(),
            costs: (1..5).map(|i| get_ressource_list(&words[i])).collect(),
        }
    };

    input.into_iter().map(|l| get_bp(&l)).collect()
}
