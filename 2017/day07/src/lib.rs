use std::{fs, collections::HashMap};

pub type InputType = HashMap<String,(u64,Vec<String>)>;

struct Tree {
    label: String,
    weight: u64,
    children: Vec<Tree>
}

/// Read output.
pub fn result_1(input: InputType) -> i64
{
    for (name,_) in input.iter() {
        let mut is_root = true;
        for (_,(_,children)) in input.iter() {
            if children.contains(name) {
                is_root = false;
                break;
            }
        }

        if is_root {
            println!("{name}");
            return 0;
        }
    }

    0
}

fn create_tree(root: String, links: &InputType) -> Tree {
    let entry = links.get(&root).unwrap();
    let c = entry.1.iter().map(|name| create_tree(name.clone(), links)).collect::<Vec<Tree>>();
    let w = entry.0 + c.iter().map(|el| el.weight).sum::<u64>();

    let tree = Tree {
        label: root,
        weight: w,
        children: c
    };

    tree
}

// (is,should_be)
fn detect_wrong_weight(t: &Tree) -> Option<(String,u64,u64)> {

    for i in 0..t.children.len() {
        if let Some(x) = detect_wrong_weight(&t.children[i]) {
            return Some(x);
        }
    }

    // cannot resolve a 2 disks conflict (who is right ?)
    // so it has to be 3 or more
    if t.children.len() > 2 {

        for i in 0..t.children.len() {
            let mut not_equal_count = 0;
            
            for j in 0..t.children.len() {
                if t.children[i].weight != t.children[j].weight {
                    not_equal_count += 1;
                }
            }

            if not_equal_count > 1 {
                return Some((
                    t.children[i].label.clone(),
                    t.children[i].weight,
                    t.children[(i+1)%t.children.len()].weight
                ));
            }
        }
    }

    None
}

pub fn result_2(input: InputType) -> i64
{   
    let mut root = "";

    for (name,_) in input.iter() {
        let mut is_root = true;
        for (_,(_,children)) in input.iter() {
            if children.contains(name) {
                is_root = false;
                break;
            }
        }

        if is_root {
            root = name;
            break; 
        }
    }

    let t = create_tree(root.to_string(), &input);

    let (name, is, should_be) = detect_wrong_weight(&t).unwrap();

    println!("{name}");

    input.get(&name).unwrap().0 as i64 + (should_be as i64 - is as i64)
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let mut res: InputType = HashMap::new();

    for l in input {
        let w = l.split_whitespace().collect::<Vec<&str>>();
        let age = w[1][1..(w[1].len()-1)].parse::<u64>().unwrap();
        let mut above: Vec<String> = vec![];

        if w.len() > 3 {
            for i in 3..w.len() {
                above.push(w[i].replace(",", ""));
            }
        }

        res.insert(w[0].to_string(),(age,above));
    }

    res
}

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn test2()
    {
        assert_eq!(result_2(read_input(TEST_INPUT_PATH)), 60);
    }
}