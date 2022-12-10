#![feature(drain_filter)]
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Clan {
    Gobelin,
    Elf,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Occupied(Clan),
    Free,
    Wall,
}

#[derive(Clone, Copy, Debug)]
pub struct Player {
    x: usize,
    y: usize,
    clan: Clan,
    hitpoints: i64,
    attack_force: usize,
}

pub struct Game {
    map: Vec<Vec<State>>,
    players: Vec<Player>,
}

impl Game {
    fn print(&self) {
        for l in self.map.iter() {
            for c in l.iter() {
                print!(
                    "{}",
                    match c {
                        State::Free => ".",
                        State::Occupied(clan) => match clan {
                            Clan::Gobelin => "G",
                            Clan::Elf => "E",
                        },
                        State::Wall => "#",
                    }
                );
            }
            println!();
        }
    }
}

pub type InputType = Game;

fn compute_accessible_squares(game: &InputType, player_index: usize) -> Vec<(usize, usize)> {
    let mut squares = vec![];

    for other in game.players.iter() {
        if other.clan != game.players[player_index].clan {
            for (x, y) in [
                (other.x as i64 + 1, other.y as i64),
                (other.x as i64 - 1, other.y as i64),
                (other.x as i64, other.y as i64 + 1),
                (other.x as i64, other.y as i64 - 1),
            ] {
                if x >= 0
                    && y >= 0
                    && x < game.map.len() as i64
                    && y < game.map[0].len() as i64
                    && game.map[x as usize][y as usize] == State::Free
                {
                    squares.push((x as usize, y as usize));
                }
            }
        }
    }

    squares
}

// returns vec of (usize,usize)
fn compute_ennemies_in_range(game: &InputType, player_index: usize) -> Vec<usize> {
    [
        (
            game.players[player_index].x as i64 + 1,
            game.players[player_index].y as i64,
        ),
        (
            game.players[player_index].x as i64 - 1,
            game.players[player_index].y as i64,
        ),
        (
            game.players[player_index].x as i64,
            game.players[player_index].y as i64 + 1,
        ),
        (
            game.players[player_index].x as i64,
            game.players[player_index].y as i64 - 1,
        ),
    ]
    .into_iter()
    .filter(|&(x, y)| x >= 0 && y >= 0 && x < game.map.len() as i64 && y < game.map[0].len() as i64)
    .map(|(x, y)| (x as usize, y as usize))
    .filter(|&(x, y)| match game.map[x][y] {
        State::Occupied(c) => c != game.players[player_index].clan,
        _ => false,
    })
    .map(|(x, y)| {
        game.players
            .iter()
            .enumerate()
            .filter(|(_, p)| p.x == x && p.y == y)
            .next()
            .unwrap()
            .0
    })
    .collect()
}

fn compute_best_move(
    game: &InputType,
    start: (usize, usize),
    goals: Vec<(usize, usize)>,
) -> Option<u8> {
    // grid of (distance from start, vec of all best paths to get there)
    let (h, w) = (game.map.len(), game.map[0].len());
    let mut scores: Vec<Vec<(usize, Vec<Vec<(usize, usize)>>)>> =
        vec![vec![(usize::MAX, vec![]); w]; h];
    scores[start.0][start.1] = (0, vec![vec![start; 1]; 1]);
    // score, position, best paths to get there
    let mut queue: Vec<(usize, (usize, usize))> = vec![];
    queue.push((0, (start.0, start.1)));
    let mut min_length_found = usize::MAX;

    while !queue.is_empty() {
        queue.sort_by_key(|(score, _)| -(*score as i64));
        let (score, position) = queue.pop().unwrap();

        // no more candidate path better than the one already found
        if score > min_length_found {
            break;
        }

        if goals.contains(&position) {
            min_length_found = score;
        }

        if min_length_found != usize::MAX {
            // all potential other candidates should be in the queue now
            continue;
        }

        let mut temp_neighbors = Vec::from([
            (position.0 as i64 - 1, position.1 as i64),
            (position.0 as i64 + 1, position.1 as i64),
            (position.0 as i64, position.1 as i64 - 1),
            (position.0 as i64, position.1 as i64 + 1),
        ]);

        // temp_neighbors.drain_filter(|&mut (x, y)| {
        //     !(x >= 0
        //         && y >= 0
        //         && x < h as i64
        //         && y < w as i64
        //         && game.map[x as usize][y as usize].eq(&State::Free))
        // });
        temp_neighbors = temp_neighbors
            .into_iter()
            .filter(|&(x, y)| {
                x >= 0
                    && y >= 0
                    && x < h as i64
                    && y < w as i64
                    && game.map[x as usize][y as usize].eq(&State::Free)
            })
            .collect();

        let neighbors = temp_neighbors
            .into_iter()
            .map(|(x, y)| (x as usize, y as usize))
            .collect::<Vec<(usize, usize)>>();

        for (x, y) in neighbors {
            let score_diff = scores[x][y].0 as i128 - (score + 1) as i128;
            if score_diff > 0 {
                scores[x][y] = (
                    score + 1,
                    scores[position.0][position.1]
                        .1
                        .iter()
                        .map(|path| {
                            let mut new_path = path.clone();
                            new_path.push((x, y));
                            new_path
                        })
                        .collect(),
                );
                queue.push((score + 1, (x, y)));
            } else if score_diff == 0 {
                let mut new_paths = scores[position.0][position.1]
                    .1
                    .iter()
                    .map(|path| {
                        let mut new_path = path.clone();
                        new_path.push((x, y));
                        new_path
                    })
                    .collect();
                scores[x][y].1.append(&mut new_paths);
            }
        }
    }

    let candidate_paths: Vec<Vec<(usize, usize)>> = goals
        .into_iter()
        .flat_map(|(x, y)| scores[x][y].1.clone())
        .collect();

    // no path
    if candidate_paths.is_empty() {
        return None;
    }

    // let min_length = candidate_paths.iter().map(|p| p.len()).min().unwrap();

    // candidate_paths.drain_filter(|p| p.len() != min_length);
    let mut possible_first_moves: Vec<u8> = candidate_paths
        .into_iter()
        .map(|p| {
            let first_move = p[1];
            if first_move.0 > start.0 {
                2
            } else if first_move.0 < start.0 {
                0
            } else if first_move.1 > start.1 {
                1
            } else if first_move.1 < start.1 {
                3
            } else {
                panic!()
            }
        })
        .collect();

    possible_first_moves.sort_by_key(|m| match m {
        0 => 3,
        3 => 2,
        1 => 1,
        2 => 0,
        _ => panic!(),
    });

    possible_first_moves.pop()
}

fn iter_game(game: &mut InputType) -> bool {
    game.players.sort_by_key(|p| p.x * game.map.len() + p.y);

    let mut i = 0;

    while i < game.players.len() {
        // dead
        if game.players[i].hitpoints <= 0 {
            i += 1;
            continue;
        }
        let accessible_squares = compute_accessible_squares(game, i);
        let mut ennemies_in_range = compute_ennemies_in_range(game, i);
        if ennemies_in_range.is_empty() && accessible_squares.is_empty() {
            // nothing to do
            i += 1;
            continue;
        } else if ennemies_in_range.is_empty() {
            // tries to move
            let best_move = compute_best_move(
                game,
                (game.players[i].x, game.players[i].y),
                accessible_squares,
            );
            if best_move.is_none() {
                // no reachable target
                i += 1;
                continue;
            }
            // actually moves
            game.map[game.players[i].x][game.players[i].y] = State::Free;
            match best_move.unwrap() {
                0 => game.players[i].x -= 1,
                1 => game.players[i].y += 1,
                2 => game.players[i].x += 1,
                3 => game.players[i].y -= 1,
                _ => panic!(),
            }
            game.map[game.players[i].x][game.players[i].y] = State::Occupied(game.players[i].clan);
            ennemies_in_range = compute_ennemies_in_range(game, i);
        }

        if !ennemies_in_range.is_empty() {
            // attacks
            ennemies_in_range.sort_by_key(|&j| {
                -(game.players[j].hitpoints * 100000
                    + (game.players[j].x * game.map.len()) as i64
                    + game.players[j].y as i64)
            });

            let victim_id = ennemies_in_range.pop().unwrap();

            game.players[victim_id].hitpoints -= game.players[i].attack_force as i64;

            // victim dies
            if game.players[victim_id].hitpoints <= 0 {
                game.map[game.players[victim_id].x][game.players[victim_id].y] = State::Free;
                game.players.remove(victim_id);
                if victim_id < i {
                    i -= 1;
                }
            }
        }

        i += 1;
    }

    // game ends if only one clan remains
    let clan = game.players[0].clan;

    for p in game.players.iter() {
        if p.clan != clan {
            return false;
        }
    }

    true
}

pub fn result_1(mut game: InputType) -> i64 {
    let mut round = 0;
    while !iter_game(&mut game) {
        println!("{}", round + 1);
        game.print();
        round += 1;
    }
    println!(
        "{} * {}",
        round,
        game.players.iter().map(|p| p.hitpoints).sum::<i64>()
    );
    game.print();
    game.players.iter().map(|p| p.hitpoints).sum::<i64>() * round
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

    let map = input
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => State::Wall,
                    '.' => State::Free,
                    'G' => State::Occupied(Clan::Gobelin),
                    'E' => State::Occupied(Clan::Elf),
                    _ => panic!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<State>>>();

    let mut players = vec![];

    (0..map.len()).for_each(|i| {
        (0..map[0].len()).for_each(|j| match map[i][j] {
            State::Occupied(c) => players.push(Player {
                x: i,
                y: j,
                clan: c,
                hitpoints: 200,
                attack_force: 3,
            }),
            _ => (),
        })
    });

    Game { map, players }
}
