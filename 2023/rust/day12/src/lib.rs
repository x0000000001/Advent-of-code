mod helpers;

use std::collections::HashMap;

pub use helpers::Solution;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Spring {
    Good,
    Broken,
    Unknown,
}

type InputType = Vec<(Vec<Spring>, Vec<usize>)>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State {
    remaining_count: usize,
    needed_broken_count: usize,
    current_group_id: i64,
    current_position: i64,
    current_group_count: usize,
    force_current_spring: Option<Spring>,
}

fn possibilities_smart(
    springs: &Vec<Spring>,
    groups: &Vec<usize>,
    state: State,
    seen: &mut HashMap<State, usize>,
) -> usize {
    if let Some(&x) = seen.get(&state) {
        return x;
    }

    if state.current_position < 0 {
        if state.current_group_id < 0
            || (state.current_group_id == 0 && state.current_group_count == groups[0])
        {
            return 1;
        } else {
            return 0;
        }
    }

    let spring = state
        .force_current_spring
        .unwrap_or(springs[state.current_position as usize]);

    match spring {
        Spring::Broken => {
            if state.current_group_id < 0 {
                seen.insert(state, 0);
                return 0;
            }

            let res = possibilities_smart(
                springs,
                groups,
                State {
                    remaining_count: state.remaining_count,
                    needed_broken_count: state.needed_broken_count,
                    current_group_id: state.current_group_id,
                    current_position: state.current_position - 1,
                    current_group_count: state.current_group_count + 1,
                    force_current_spring: None,
                },
                seen,
            );

            seen.insert(state, res);
            res
        }
        Spring::Good => {
            let mut next_group_id = state.current_group_id;

            if state.current_group_count > 0 {
                if state.current_group_id < 0
                    || state.current_group_count != groups[state.current_group_id as usize]
                {
                    seen.insert(state, 0);
                    return 0;
                }

                next_group_id -= 1;
            }

            let res = possibilities_smart(
                springs,
                groups,
                State {
                    remaining_count: state.remaining_count,
                    needed_broken_count: state.needed_broken_count,
                    current_group_id: next_group_id,
                    current_position: state.current_position - 1,
                    current_group_count: 0,
                    force_current_spring: None,
                },
                seen,
            );

            seen.insert(state, res);
            res
        }
        Spring::Unknown => {
            let mut can_be_broken = true;
            can_be_broken &= state.remaining_count > 0 && state.needed_broken_count > 0;

            let mut can_be_good = true;
            can_be_good &= state.remaining_count > state.needed_broken_count;

            if state.current_group_count > 0 {
                let needed_group_count = groups[state.current_group_id as usize];
                can_be_broken &= state.current_group_count < needed_group_count;
                can_be_good &= state.current_group_count == needed_group_count;
            }

            let res = (if can_be_broken {
                possibilities_smart(
                    springs,
                    groups,
                    State {
                        remaining_count: state.remaining_count - 1,
                        needed_broken_count: state.needed_broken_count - 1,
                        current_group_id: state.current_group_id,
                        current_position: state.current_position,
                        current_group_count: state.current_group_count,
                        force_current_spring: Some(Spring::Broken),
                    },
                    seen,
                )
            } else {
                0
            }) + (if can_be_good {
                possibilities_smart(
                    springs,
                    groups,
                    State {
                        remaining_count: state.remaining_count - 1,
                        needed_broken_count: state.needed_broken_count,
                        current_group_id: state.current_group_id,
                        current_position: state.current_position,
                        current_group_count: state.current_group_count,
                        force_current_spring: Some(Spring::Good),
                    },
                    seen,
                )
            } else {
                0
            });

            seen.insert(state, res);
            res
        }
    }
}

fn count_possibilities(springs: &Vec<Spring>, groups: &Vec<usize>) -> usize {
    let unknown_count = springs
        .iter()
        .filter(|s| matches!(s, Spring::Unknown))
        .count();
    let broken_count = springs
        .iter()
        .filter(|s| matches!(s, Spring::Broken))
        .count();
    let total_needed_broken_count = groups.iter().sum::<usize>();

    let start_state = State {
        remaining_count: unknown_count,
        needed_broken_count: total_needed_broken_count - broken_count,
        current_group_id: groups.len() as i64 - 1,
        current_position: springs.len() as i64 - 1,
        current_group_count: 0,
        force_current_spring: None,
    };

    let mut seen = HashMap::new();

    possibilities_smart(springs, groups, start_state, &mut seen)
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    Solution::from(
        input
            .iter()
            .map(|(springs, groups)| count_possibilities(springs, groups))
            .sum::<usize>() as u64,
    )
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);
    input = input
        .into_iter()
        .map(|(springs, groups)| {
            (
                (0..5)
                    .map(|_| springs.clone())
                    .collect::<Vec<Vec<Spring>>>()
                    .join(&Spring::Unknown),
                groups.repeat(5),
            )
        })
        .collect();

    Solution::from(
        input
            .iter()
            .map(|(springs, groups)| count_possibilities(springs, groups))
            .sum::<usize>() as u64,
    )
}

fn parse(s: String) -> InputType {
    s.lines()
        .map(|line| {
            let w = line.split_whitespace().collect::<Vec<&str>>();

            (
                w[0].chars()
                    .map(|c| match c {
                        '?' => Spring::Unknown,
                        '#' => Spring::Broken,
                        '.' => Spring::Good,
                        _ => panic!(),
                    })
                    .collect(),
                w[1].split(',').map(|s| s.parse().unwrap()).collect(),
            )
        })
        .collect()
}
