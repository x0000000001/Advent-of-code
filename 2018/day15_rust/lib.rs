#![feature(drain_filter)]
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Clan {
    Gobelin,
    Elf,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Occupied(Clan),
    Free,
    Wall,
}

#[derive(Clone, Copy)]
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

pub type InputType = Game;

fn accesible_squares(game: &InputType, player_index: usize) -> Vec<(usize, usize)> {
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

// returns vec of ids
fn ennemies_in_range(game: &InputType, player_index: usize) -> Vec<usize> {
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

/// returns (
///    Option<length of shortest path>,
///    best orientation (0,1,2,3) to follow this shortest path, by reading order
///)
fn shortest_path_routes(
    game: &InputType,
    start: (usize, usize),
    goal: (usize, usize),
) -> (Option<usize>, u8) {
    todo!()
}

fn iter_game(game: &mut InputType) -> bool {
    game.players.sort_by_key(|p| p.x * game.map.len() + p.y);

    let mut i = 0;

    while i < game.players.len() {
        // dead
        if game.players[i].hitpoints <= 0 {
            continue;
        }
        let squares = accesible_squares(game, i);
        let mut range_ennemies = ennemies_in_range(game, i);
        if range_ennemies.is_empty() && squares.is_empty() {
            // nothing to do
            continue;
        } else if range_ennemies.is_empty() {
            // tries to move
            let mut moves: Vec<(Option<usize>, u8)> = squares
                .iter()
                .map(|goal| {
                    shortest_path_routes(game, (game.players[i].x, game.players[i].y), *goal)
                })
                .filter(|(length, _)| length.is_some())
                .collect();

            if moves.is_empty() {
                // no reachable target
                continue;
            }
            // moves
            moves.sort_by_key(|(length, _)| *length);
            game.map[game.players[i].x][game.players[i].y] = State::Free;
            match moves[0].1 {
                0 => game.players[i].x -= 1,
                1 => game.players[i].y += 1,
                2 => game.players[i].x += 1,
                3 => game.players[i].y -= 1,
                _ => panic!(),
            }
            game.map[game.players[i].x][game.players[i].y] = State::Occupied(game.players[i].clan);
        }

        range_ennemies = ennemies_in_range(game, i);
        if !range_ennemies.is_empty() {
            // attacks
            range_ennemies.sort_by_key(|j| {
                1000000 - game.players[*j].hitpoints as usize
                    + game.players[*j].x * game.map.len()
                    + game.players[*j].y
            });

            game.players[range_ennemies[0]].hitpoints -= game.players[i].attack_force as i64;

            // attacked player dies
            if game.players[range_ennemies[0]].hitpoints <= 0 {
                game.players.remove(range_ennemies[0]);
                if range_ennemies[0] < i {
                    i -= 1;
                }
            }
        }

        i += 1;
    }

    false
}

pub fn result_1(mut game: InputType) -> i64 {
    let mut round = 0;
    while !iter_game(&mut game) {
        round += 1;
    }
    0
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
