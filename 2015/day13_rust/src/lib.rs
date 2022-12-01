use std::{cmp::Ordering, collections::HashMap};

pub type InputType = Vec<(String,String,i64)>;

/// Same djisktra's modified algorithm as 2015::day9,
/// except this time add the weoght of the last node visited to the first node.
fn best_path_cost(weights: &mut Vec<Vec<Option<i64>>>, compare_func: fn(i64,i64) -> Ordering) -> i64 {
    // node, ,visited, cost
    let mut costs: HashMap<(usize, u64),i64> = HashMap::new();
    // node, visited
    let mut queue: Vec<(usize, u64)> = vec![];
    let len = weights.len();

    // 1 = 2^0
    queue.push((0,1));
    costs.insert((0,1), 0);

    while !queue.is_empty() {
        queue.sort_by(|(_, c0), (_, c1)| compare_func(*c0 as i64,*c1 as i64));
        let (node, visited) = queue.swap_remove(0);
        let cost = costs.get(&(node, visited)).unwrap().clone();
        
        for j in 0..len {
            // must check if not already visited
            if (visited >> j)%2 == 1 {
                continue;
            }

            if let Some(weight) = weights[node][j]{
                let new_visited = visited + 2_u64.pow(j as u32);
                let ancient_cost = costs.get(&(j, new_visited));

                if ancient_cost.is_none() || compare_func(*ancient_cost.unwrap(), cost + weight) == Ordering::Greater {
                    let accessor = costs.entry((j, new_visited)).or_insert(0);
                    *accessor = cost + weight;
                    queue.push((j, new_visited));
                }
            } 
        }
    }

    let mut max = 0;
    let all_visited = 2_u64.pow(len as u32) - 1;

    for ((node, visited), c) in costs {
        if visited != all_visited {
            continue;
        }
        let candidate = c + weights[node][0].unwrap();
        if candidate > max {
            max = candidate;
        }
    }

    max
}

fn transform_input(input: &InputType) -> Vec<Vec<Option<i64>>> {
    let mut i = 0;
    let mut names_to_num: HashMap<&str, u64> = HashMap::new();

    for (city0,_,_) in input.iter() {
        if !names_to_num.contains_key(&city0[..]) {
            names_to_num.insert(city0, i);
            
            i += 1;
        }
    }

    let mut new_weights: Vec<Vec<Option<i64>>> = vec![vec![None;i as usize];i as usize];

    for (city0,city1,v) in input.iter() {
        let i = *names_to_num.get(&city0[..]).unwrap() as usize;
        let j = *names_to_num.get(&city1[..]).unwrap() as usize;
        // unoriented graph
        let c = new_weights[i][j].unwrap_or(0);
        new_weights[i][j] = Some(c + *v);
        let c = new_weights[j][i].unwrap_or(0);
        new_weights[j][i] = Some(c + *v);
    }

    new_weights
}

pub fn result_1(input: InputType) -> i64
{
    best_path_cost(&mut transform_input(&input), 
        |x: i64, y: i64| -> Ordering {y.partial_cmp(&x).unwrap()})
}

pub fn result_2(input: InputType) -> i64
{  
    let mut d = transform_input(&input);
    d.push(vec![Some(0);d.len()]);
    let len = d.len();
    for i in 0..len-1 {
        d[i].push(Some(0));
    }

    d[len-1].push(None);

    best_path_cost(&mut d, 
        |x: i64, y: i64| -> Ordering {y.partial_cmp(&x).unwrap()})
}