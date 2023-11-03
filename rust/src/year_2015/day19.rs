use crate::Solution;
use std::collections::HashMap;

type InputType = (String, HashMap<String, Vec<String>>);

fn get_next(molecule: &String, combinations: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut already_seen: HashMap<String, u8> = HashMap::new();

    for (pattern, replacements) in combinations.iter() {
        let indices = molecule
            .match_indices(pattern)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        for r in replacements {
            for i in indices.iter() {
                let mut new = molecule.clone();
                new.replace_range(*i..(*i + pattern.len()), r);

                already_seen.insert(new, 0);
            }
        }
    }

    already_seen.into_iter().map(|(k, _)| k).collect()
}

pub fn part1(s: String) -> Solution {
    let (molecule, combinations) = parse(s);

    Solution::from(get_next(&molecule, &combinations).into_iter().count() as i64)
}

pub fn part2(s: String) -> Solution {
    let (molecule_init, combinations_init) = parse(s);

    let mut comb: Vec<(String, String)> = vec![];

    for (pattern, replacements) in combinations_init {
        for r in replacements {
            comb.push((r, pattern.clone()));
        }
    }

    comb.sort_by(|(x0, _), (x1, _)| x1.len().partial_cmp(&x0.len()).unwrap());

    let mut seen: HashMap<String, u8> = HashMap::new();
    let mut curr = molecule_init;

    let mut c = 0;

    while curr != "e".to_string() {
        c += 1;

        for (s0, s1) in comb.iter() {
            if curr.find(s0).is_some() {
                seen.insert(curr.clone(), 0);
                curr = curr.replacen(s0, s1, 1);
                break;
            }
        }
    }

    Solution::from(c)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut combinations: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..input.len() - 2 {
        let words = input[i].split_whitespace().collect::<Vec<&str>>();

        let mol = words[0].to_string();
        let follow = words[2].to_string();

        let accessor = combinations.entry(mol).or_insert(vec![]);
        accessor.push(follow);
    }

    (input.last().unwrap().clone(), combinations)
}
