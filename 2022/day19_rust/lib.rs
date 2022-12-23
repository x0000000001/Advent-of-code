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
) -> usize {
    assert_eq!(ores.len(), 4);
    assert_eq!(robots.len(), 4);

    if *time == 0 {
        return ores[3];
    }

    let mut max = 0;
    let new_ores = ores
        .iter()
        .enumerate()
        .map(|(i, &o)| o + robots[i])
        .collect::<Ores>();
    let new_time = time - 1;

    let mut neighbors = vec![];

    neighbors.push((new_time, new_ores.clone(), robots.clone()));

    for i in 0..4 {
        if (0..4)
            .map(|j| ores[j] >= bp.costs[i][j])
            .reduce(|a, b| a && b)
            .unwrap()
        {
            let n_ores = new_ores
                .iter()
                .enumerate()
                .map(|(j, &o)| o - bp.costs[i][j])
                .collect();
            let mut n_robots = robots.clone();
            n_robots[i] += 1;

            neighbors.push((new_time, n_ores, n_robots));
        }
    }

    for n in neighbors {
        let n_s;
        if let Some(&s) = seen.get(&n) {
            n_s = s;
        } else {
            n_s = blueprint_score(bp, seen, &n);
            seen.insert(n, n_s);
        }
        max = max.max(n_s);
    }

    max
}

pub fn result_1(input: InputType) -> i64 {
    input
        .into_iter()
        .map(|bp| {
            bp.id
                * blueprint_score(
                    &bp,
                    &mut HashMap::new(),
                    &(24, vec![0; 4], Vec::from([1, 0, 0, 0])),
                )
        })
        .sum::<usize>() as i64
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
