mod helpers;

use std::usize;

pub use helpers::Solution;

struct Game {
    id: usize,
    rounds: Vec<(usize, usize, usize)>,
}

type InputType = Vec<Game>;

pub fn part1(s: String) -> Solution {
    let games = parse(s);

    Solution::from(
        games
            .iter()
            .filter_map(|g| {
                for &(r, g, b) in g.rounds.iter() {
                    if r > 12 || g > 13 || b > 14 {
                        return None;
                    }
                }

                Some(g.id)
            })
            .sum::<usize>() as u64,
    )
}

pub fn part2(s: String) -> Solution {
    let games = parse(s);

    Solution::from(
        games
            .iter()
            .map(|g| {
                let (r_max, g_max, b_max) = g
                    .rounds
                    .iter()
                    .fold((0, 0, 0), |(r_max, g_max, b_max), &(r, g, b)| {
                        (r_max.max(r), g_max.max(g), b_max.max(b))
                    });

                r_max * g_max * b_max
            })
            .sum::<usize>() as u64,
    )
}

fn parse(s: String) -> InputType {
    let mut games = vec![];

    for line in s.lines() {
        let w0 = line.split(':').collect::<Vec<&str>>();
        let id = w0[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut rounds = vec![];

        for s_round in w0[1].split(';') {
            let mut vals = (0, 0, 0);

            for s_val in s_round.split(',') {
                let w = s_val.split_whitespace().collect::<Vec<&str>>();
                let v = w[0].parse::<usize>().unwrap();

                match w[1] {
                    "red" => vals.0 = v,
                    "green" => vals.1 = v,
                    "blue" => vals.2 = v,
                    _ => panic!(),
                }
            }

            rounds.push(vals)
        }

        games.push(Game { id, rounds })
    }

    games
}
