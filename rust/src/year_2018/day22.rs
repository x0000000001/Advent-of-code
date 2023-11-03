use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::Solution;

type InputType = (usize, Position);

pub type Position = (usize, usize);

const CX: usize = 16807;
const CY: usize = 48271;
const MOD: usize = 20183;

fn terrains(depth: usize, target: Position) -> Vec<Vec<usize>> {
    let mut geological = vec![vec![0; target.0 + 1]; target.1 + 1];
    let mut erosion = vec![vec![0; target.0 + 1]; target.1 + 1];

    for y in 0..=target.1 {
        for x in 0..=target.0 {
            geological[y][x] = match (x, y) {
                (0, 0) => 0,
                (x, y) if (x, y) == target => 0,
                (0, y) => (y * CY) % MOD,
                (x, 0) => (x * CX) % MOD,
                (x, y) => erosion[y][x - 1] * erosion[y - 1][x],
            };

            erosion[y][x] = (geological[y][x] + depth) % MOD;
        }
    }

    for y in 0..=target.1 {
        for x in 0..=target.0 {
            erosion[y][x] = erosion[y][x] % 3;
        }
    }

    erosion
}

pub fn part1(s: String) -> Solution {
    let (depth, target) = parse(s);

    let t = terrains(depth, target);

    Solution::from(
        (0..=target.1)
            .map(|y| (0..=target.0).map(|x| t[y][x]).sum::<usize>())
            .sum::<usize>() as i64,
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tool {
    Torch,
    Climbing,
    Neither,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    pos: Position,
    tool: Tool,
}

#[derive(Debug, Hash, Clone, Copy)]
struct StateScore {
    state: State,
    score: usize,
}

impl PartialEq for StateScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for StateScore {}

impl PartialOrd for StateScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Ord for StateScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn manhanttan((x0, y0): Position, (x1, y1): Position) -> usize {
    (x0.max(x1) - x0.min(x1)) + (y0.max(y1) - y0.min(y1))
}

fn djisktra(
    t: &Vec<Vec<usize>>,
    target: Position,
    allowed_tools: &HashMap<usize, Vec<Tool>>,
) -> usize {
    let mut seen = HashSet::new();
    let mut scores = HashMap::new();
    let mut queue = BinaryHeap::new();

    let start_state = State {
        pos: (0, 0),
        tool: Tool::Torch,
    };

    scores.insert(start_state, 0);
    queue.push(StateScore {
        state: start_state,
        score: 0,
    });

    while let Some(StateScore {
        state: state @ State { pos: (x, y), tool },
        score: _,
    }) = queue.pop()
    {
        let score = *scores.get(&state).unwrap();

        if state.pos == target && tool == Tool::Torch {
            return score;
        }

        if seen.contains(&state) {
            continue;
        }

        seen.insert(state);

        let mut neighbors_pos = vec![];
        if x > 0 {
            neighbors_pos.push((x - 1, y));
        };
        neighbors_pos.push((x + 1, y));
        if y > 0 {
            neighbors_pos.push((x, y - 1));
        };
        neighbors_pos.push((x, y + 1));

        let other_tools = allowed_tools
            .get(&t[y][x])
            .unwrap()
            .iter()
            .filter(|&&t| t != tool)
            .collect::<Vec<&Tool>>();

        assert_eq!(other_tools.len(), 1);

        // useless to change tool if can't go in any neighbor with it
        let useful = neighbors_pos
            .iter()
            .map(|&(yn, xn)| allowed_tools.get(&t[yn][xn]).unwrap().contains(&tool))
            .fold(false, |acc, b| acc || b);

        let mut potential_neighbors = neighbors_pos
            .into_iter()
            .filter(|&(xn, yn)| allowed_tools.get(&t[yn][xn]).unwrap().contains(&tool))
            .map(|posn| {
                (
                    State {
                        pos: posn,
                        tool: tool,
                    },
                    score + 1,
                )
            })
            .collect::<Vec<(State, usize)>>();

        if useful {
            potential_neighbors.push((
                State {
                    pos: state.pos,
                    tool: *other_tools[0],
                },
                score + 7,
            ));
        }

        for (potential_neighbor, new_score) in potential_neighbors {
            if seen.contains(&potential_neighbor) {
                continue;
            } else if let Some(&old_score) = scores.get(&potential_neighbor) {
                if old_score <= new_score {
                    continue;
                }
            }

            scores.insert(potential_neighbor, new_score);

            queue.push(StateScore {
                state: potential_neighbor,
                score: new_score + manhanttan(potential_neighbor.pos, target),
            });
        }
    }

    0
}

pub fn part2(s: String) -> Solution {
    let (depth, target) = parse(s);

    let mut t = terrains(depth, (target.0 * 100, target.1 * 10));
    t[target.1][target.0] = 0;

    let allowed_tools: HashMap<usize, Vec<Tool>> = HashMap::from_iter(
        [
            (0, vec![Tool::Climbing, Tool::Torch]),
            (1, vec![Tool::Climbing, Tool::Neither]),
            (2, vec![Tool::Torch, Tool::Neither]),
        ]
        .into_iter(),
    );

    Solution::from(djisktra(&t, target, &allowed_tools) as i64)
}

fn parse(s: String) -> InputType {
    let lines: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();
    let depth = lines[0].split_ascii_whitespace().collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    let xy = lines[1].split_ascii_whitespace().collect::<Vec<&str>>()[1]
        .split(',')
        .map(|w| w.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let p = (xy[0], xy[1]);

    (depth, p)
}
