use std::fs;

const FILE: &str = "input.txt";

// This algorithm is not optimized at all :
// Strings are being copied everywhere, it is dumb

fn get_paths(current_point: String, links: Vec<(String, String)>) -> Vec<Vec<String>> {
    if current_point == "end" {
        return vec![vec![current_point; 1]; 1];
    }

    let mut paths: Vec<Vec<String>> = Vec::new();

    for link in links.iter() {
        let mut next: Option<&str> = None;

        if link.0 == current_point {
            next = Some(&link.1);
        }

        if let Some(next_val) = next {
            let mut new_available_links = links.clone();
            //cannot come back if lowercase letters
            if next_val.to_uppercase() != next_val {
                new_available_links.retain(|el| el.1 != String::from(next_val));
            }

            let new_potential_ends = get_paths(String::from(next_val), new_available_links);
            for e in new_potential_ends {
                if e.last().unwrap() == "end" {
                    let mut e_completed = e.clone();
                    e_completed.insert(0, current_point.clone());
                    paths.push(e_completed);
                }
            }
        }
    }

    paths
}

pub fn result_1() -> i32 {
    let contents = fs::read_to_string(FILE).expect("Something went wrong reading the file");

    let input: Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let mut links: Vec<(String, String)> = input
        .iter()
        .map(|l| {
            let w = l.split('-').map(|w| String::from(w)).collect::<Vec<_>>();
            (w[0].clone(), w[1].clone())
        })
        .collect::<Vec<(_, _)>>();

    //making bridges assymetrical
    for i in 0..links.len() {
        links.push((links[i].1.clone(), links[i].0.clone()));
    }

    //cannot come back to start
    links.retain(|el| el.1 != "start");

    //println!("{:?}", links);

    let paths = get_paths(String::from("start"), links);

    //println!("{:?}", paths);

    paths.len() as i32
}

// This is BAD. Very bad. To be clear : the algorithm is optimized. But the way storage is handled is HORRIBLE :
// everything just gets copied into Strings, which are very heavy for the computer at runtime.
fn get_paths2(
    current_point: String,
    links: Vec<(String, String)>,
    previous_path: Vec<String>,
    small_cave_twice: bool,
) -> Vec<Vec<String>> {
    if current_point == "end" {
        return vec![vec![current_point; 1]; 1];
    }

    let mut paths: Vec<Vec<String>> = Vec::new();

    for link in links.iter() {
        let mut next: Option<&str> = None;

        if link.0 == current_point {
            next = Some(&link.1);
        }

        if let Some(next_val) = next {
            let mut new_path = previous_path.clone();
            new_path.insert(0, current_point.clone());

            let current_is_lower: bool = current_point.to_uppercase() != current_point;

            let new_small_cave_twice =
                small_cave_twice || (previous_path.contains(&current_point) && current_is_lower);

            let next_is_lower: bool = next_val.to_uppercase() != next_val;

            if new_small_cave_twice && next_is_lower && new_path.contains(&String::from(next_val)) {
                continue;
            }

            let mut new_available_links = links.clone();

            if new_small_cave_twice {
                if next_val.to_uppercase() != next_val {
                    new_available_links.retain(|el| el.1 != String::from(next_val));
                }

                for point in new_path.iter() {
                    if point.to_uppercase() != *point {
                        new_available_links.retain(|el| el.1 != *point);
                    }
                }
            }

            let new_potential_ends = get_paths2(
                String::from(next_val),
                new_available_links,
                new_path,
                new_small_cave_twice,
            );

            for e in new_potential_ends {
                if e.last().unwrap() != "end" {
                    continue;
                }

                let mut e_completed = e.clone();
                e_completed.insert(0, current_point.clone());
                paths.push(e_completed);
            }
        }
    }

    paths
}

pub fn result_2() -> i32 {
    let contents = fs::read_to_string(FILE).expect("Something went wrong reading the file");

    let input: Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let mut links: Vec<(String, String)> = input
        .iter()
        .map(|l| {
            let w = l.split('-').map(|w| String::from(w)).collect::<Vec<_>>();
            (w[0].clone(), w[1].clone())
        })
        .collect::<Vec<(_, _)>>();

    //making bridges assymetrical
    for i in 0..links.len() {
        links.push((links[i].1.clone(), links[i].0.clone()));
    }

    //cannot come back to start
    links.retain(|el| el.1 != "start");

    // println!("{:?}", links);

    let paths = get_paths2(String::from("start"), links, Vec::new(), false);

    // for p in paths.iter() {
    //     println!("{:?}",p);
    // }

    paths.len() as i32
}
