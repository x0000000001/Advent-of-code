use std::collections::HashMap;

pub type InputType = (String, HashMap<String, Vec<String>>);

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

pub fn result_1((molecule, combinations): InputType) -> i64 {
    get_next(&molecule, &combinations).into_iter().count() as i64
}

/// Ultra greedy algorithm, not exact. \
/// * 1st trick : reversing : starting with goal molecule. \
/// * 2nd trick : sort possible replacements by length
/// and always applying the one that replaces the biggest part.
pub fn result_2((molecule_init, combinations_init): InputType) -> i64 {
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

    c
}

pub fn result_2_stock((molecule_init, combinations_init): InputType) -> i64 {
    // reversing : starting wirh goal molecule for optimization purposes
    let molecule = "e";

    let mut combinations: HashMap<String, Vec<String>> = HashMap::new();

    for (pattern, replacements) in combinations_init {
        for r in replacements {
            // // test
            // if pattern.find(&r).is_some() {
            //     continue;
            // }
            // //test

            let accessor = combinations.entry(r).or_insert(vec![]);
            accessor.push(pattern.clone());
        }
    }

    let mut already_seen: HashMap<String, u8> = HashMap::new();
    let mut current_molecules: Vec<String> = vec![molecule_init.to_string(); 1];
    let mut steps = 0;

    loop {
        println!("{:?}", current_molecules);
        let mut temp: Vec<String> = vec![];

        for m in current_molecules.iter() {
            if *m == molecule {
                return steps;
            }
        }
        steps += 1;

        for m in current_molecules.iter() {
            let nexts = get_next(&m, &combinations);

            for candidate in nexts {
                // //test
                // if candidate.len() > m.len() {
                //     continue;
                // }
                // //test

                if already_seen.get(&candidate[..]).is_none() {
                    already_seen.insert(candidate.clone(), 0);
                    temp.push(candidate);
                }
            }
        }

        current_molecules.clear();
        current_molecules.append(&mut temp);

        if current_molecules.is_empty() {
            panic!("noo");
        }
    }
}
