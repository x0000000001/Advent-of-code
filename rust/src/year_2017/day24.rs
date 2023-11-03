use std::collections::HashMap;

use crate::Solution;

type InputType = Vec<Component>;

type Component = (usize, usize);

fn strongest_bridge(
    start: usize,
    components: Vec<Component>,
    seen: &mut HashMap<Vec<Component>, usize>,
) -> usize {
    if components.is_empty() {
        return 0;
    }

    if seen.contains_key(&components) {
        return seen[&components];
    }

    let mut candidates = Vec::new();

    for c in components.iter() {
        if c.0 == start {
            let new_components = components.iter().copied().filter(|el| el != c).collect();
            candidates.push(c.0 + c.1 + strongest_bridge(c.1, new_components, seen))
        }

        if c.1 == start {
            let new_components = components.iter().copied().filter(|el| el != c).collect();
            candidates.push(c.0 + c.1 + strongest_bridge(c.0, new_components, seen))
        }
    }

    let res = candidates.into_iter().max().or(Some(0)).unwrap();

    seen.insert(components, res);

    res
}

pub fn part1(s: String) -> Solution {
    let components = parse(s);

    Solution::from(strongest_bridge(0, components, &mut HashMap::new()) as i64)
}

// depth, solidity
fn strongest_longest_bridge(
    start: usize,
    components: Vec<Component>,
    seen: &mut HashMap<Vec<Component>, (usize, usize)>,
) -> (usize, usize) {
    if components.is_empty() {
        return (0, 0);
    }

    if seen.contains_key(&components) {
        return seen[&components];
    }

    let mut candidates = Vec::new();
    let mut current_best_depth = 0;

    for c in components.iter() {
        if c.0 == start {
            let new_components = components.iter().copied().filter(|el| el != c).collect();
            let (temp_depth, mut temp_solidity) =
                strongest_longest_bridge(c.1, new_components, seen);

            temp_solidity += c.0 + c.1;

            if temp_depth > current_best_depth {
                current_best_depth = temp_depth;
                candidates = Vec::from([temp_solidity]);
            } else if current_best_depth == temp_depth {
                candidates.push(temp_solidity);
            }
        }

        if c.1 == start {
            let new_components = components.iter().copied().filter(|el| el != c).collect();
            let (temp_depth, mut temp_solidity) =
                strongest_longest_bridge(c.0, new_components, seen);
            temp_solidity += c.0 + c.1;

            if temp_depth > current_best_depth {
                current_best_depth = temp_depth;
                candidates = Vec::from([temp_solidity]);
            } else if current_best_depth == temp_depth {
                candidates.push(temp_solidity);
            }
        }
    }

    let res = candidates.into_iter().max().or(Some(0)).unwrap();

    seen.insert(components, (current_best_depth + 1, res));

    (current_best_depth + 1, res)
}

pub fn part2(s: String) -> Solution {
    let components = parse(s);

    Solution::from(strongest_longest_bridge(0, components, &mut HashMap::new()).1 as i64)
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
            let mut w = l.split("/");
            (
                w.next().unwrap().parse().unwrap(),
                w.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}
