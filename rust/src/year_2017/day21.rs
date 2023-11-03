use crate::Solution;

type Grid = Vec<Vec<bool>>;

type InputType = Vec<(Grid, Grid)>;

fn flip_vert(g: &Grid) -> Grid {
    (0..g.len())
        .map(|i| (0..g.len()).map(|j| g[g.len() - 1 - i][j]).collect())
        .collect()
}

fn flip_hoz(g: &Grid) -> Grid {
    (0..g.len())
        .map(|i| (0..g.len()).map(|j| g[i][g.len() - 1 - j]).collect())
        .collect()
}

fn rotate(g: &Grid) -> Grid {
    (0..g.len())
        .map(|i| (0..g.len()).map(|j| g[g.len() - 1 - j][i]).collect())
        .collect()
}

fn possible_grids(g: &Grid) -> Vec<Grid> {
    Vec::from([
        g.clone(),
        flip_hoz(g),
        flip_vert(g),
        rotate(g),
        rotate(&flip_hoz(g)),
        rotate(&flip_vert(g)),
        rotate(&rotate(g)),
        flip_hoz(&flip_vert(g)),
        rotate(&flip_hoz(&flip_vert(g))),
    ])
}

fn find_destination_index(result: &Grid, rules: &Vec<(Grid, Grid)>) -> usize {
    let arrangements = possible_grids(&result);
    for (i, (source, _)) in rules.iter().enumerate() {
        if arrangements.contains(&source) {
            return i;
        }
    }

    println!("not found {}", result.len());

    0
}

// fn assocs(rules: InputType) -> Vec<Vec<usize>> {
//     rules
//         .iter()
//         .map(|(_, result)| {
//             if result.len() == 4 {
//                 let (mut topleft, mut topright, mut bottomleft, mut bottomright) = (
//                     vec![vec![false; 2]; 2],
//                     vec![vec![false; 2]; 2],
//                     vec![vec![false; 2]; 2],
//                     vec![vec![false; 2]; 2],
//                 );
//                 for i in 0..2 {
//                     for j in 0..2 {
//                         topleft[i][j] = result[i][j];
//                         topright[i][j] = result[i][j + 2];
//                         bottomleft[i][j] = result[i + 2][j];
//                         bottomright[i][j] = result[i + 2][j + 2];
//                     }
//                 }
//                 Vec::from([
//                     find_destination_index(&topleft, &rules),
//                     find_destination_index(&topright, &rules),
//                     find_destination_index(&bottomleft, &rules),
//                     find_destination_index(&bottomright, &rules),
//                 ])
//             } else {
//                 Vec::from([find_destination_index(result, &rules)])
//             }
//         })
//         .collect()
// }

fn grid_count(g: &Grid) -> usize {
    g.iter()
        .flat_map(|l| l.iter().map(|&c| if c { 1 } else { 0 }))
        .sum()
}

// This was an attempt at optimizing, but doesn't work

// fn count_after_steps(
//     assocs: Vec<Vec<usize>>,
//     counts: Vec<usize>,
//     steps: usize,
//     init_index: usize,
// ) -> usize {
//     let mut presents = vec![0; assocs.len()];
//     presents[init_index] = 1;

//     for _ in 0..steps {
//         let mut temp = vec![0; assocs.len()];

//         for (k, v) in presents.iter().enumerate() {
//             if *v == 0 {
//                 continue;
//             }
//             for index in assocs[k].iter() {
//                 temp[*index] += *v;
//             }
//         }

//         presents = temp;
//     }

//     presents
//         .iter()
//         .enumerate()
//         .map(|(i, v)| counts[i] * *v)
//         .sum()
// }

// fn print_grid(g: &Grid) {
//     for i in 0..g.len() {
//         for j in 0..g.len() {
//             print!("{}", if g[i][j] { "#" } else { "." });
//         }

//         println!()
//     }
//     println!()
// }

fn enhance(g: Grid, rules: &InputType) -> Grid {
    if g.len() % 2 == 0 {
        let n = g.len() / 2 + g.len();
        let mut new_g = vec![vec![false; n]; n];

        for i in 0..g.len() / 2 {
            for j in 0..g.len() / 2 {
                let mut pattern = vec![vec![false; 2]; 2];
                for k in 0..2 {
                    for l in 0..2 {
                        pattern[k][l] = g[i * 2 + k][j * 2 + l]
                    }
                }

                let res_index = find_destination_index(&pattern, &rules);

                for k in 0..3 {
                    for l in 0..3 {
                        new_g[i * 3 + k][j * 3 + l] = rules[res_index].1[k][l];
                    }
                }
            }
        }

        new_g
    } else {
        let n = g.len() / 3 + g.len();
        let mut new_g = vec![vec![false; n]; n];

        for i in 0..g.len() / 3 {
            for j in 0..g.len() / 3 {
                let mut pattern = vec![vec![false; 3]; 3];
                for k in 0..3 {
                    for l in 0..3 {
                        pattern[k][l] = g[i * 3 + k][j * 3 + l]
                    }
                }

                let res_index = find_destination_index(&pattern, &rules);

                for k in 0..4 {
                    for l in 0..4 {
                        new_g[i * 4 + k][j * 4 + l] = rules[res_index].1[k][l];
                    }
                }
            }
        }

        new_g
    }
}

pub fn part1(s: String) -> Solution {
    let rules = parse(s);

    let mut grid = Vec::from([
        Vec::from([false, true, false]),
        Vec::from([false, false, true]),
        Vec::from([true, true, true]),
    ]);
    for _ in 0..5 {
        grid = enhance(grid, &rules);
    }

    Solution::from(grid_count(&grid) as i64)
}

pub fn part2(s: String) -> Solution {
    let rules = parse(s);

    let mut grid = Vec::from([
        Vec::from([false, true, false]),
        Vec::from([false, false, true]),
        Vec::from([true, true, true]),
    ]);

    for _ in 0..18 {
        grid = enhance(grid, &rules);
    }

    Solution::from(grid_count(&grid) as i64)
}

fn grid_from(s: &str) -> Grid {
    s.split("/")
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input
        .iter()
        .map(|l| {
            let words = l.split(" => ").collect::<Vec<&str>>();
            (grid_from(words[0]), grid_from(words[1]))
        })
        .collect()
}
