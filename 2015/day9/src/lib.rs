use std::{collections::HashMap, cmp::Ordering};

pub type InputType = Vec<(String,String,i64)>;

/// Djisktra modified algorithm. \
/// Paths are generated from all points, and costs are stored depending on which nodes have been visited.
fn best_path_cost(weights: &Vec<Vec<Option<i64>>>, compare_func: fn(i64,i64) -> Ordering) -> i64 {
    let len = weights.len();

    let mut costs: Vec<HashMap<u64,i64>> = vec![HashMap::new(); len];
    // node, visited
    let mut queue: Vec<(usize, u64)> = vec![];

    for i in 0..len {
        let visited = 2_u64.pow(i as u32);
        queue.push((i, visited.clone()));
        costs[i].insert(visited, 0);
    }

    while !queue.is_empty() {
        queue.sort_by(|(_, c0), (_, c1)| compare_func(*c0 as i64,*c1 as i64));
        let (node, visited) = queue.swap_remove(0);
        let cost = costs[node].get(&visited).unwrap().clone();
        
        for j in 0..len {
            // must check if not already visited
            if (visited >> j)%2 == 1 {
                continue;
            }

            if let Some(weight) = weights[node][j]{
                let mut ancient_cost = None;
                let new_visited = visited + 2_u64.pow(j as u32);
                if let Some(v) = costs[j].get(&new_visited) {
                    ancient_cost = Some(*v);
                }

                if ancient_cost.is_none() || compare_func(ancient_cost.unwrap(), cost + weight) == Ordering::Greater {
                    let accessor = costs[j].entry(new_visited).or_insert(0);
                    *accessor = cost + weight;
                    queue.push((j, new_visited));
                }
            } 
        }
    }

    let mut candidate = None;
    let all_nodes_visited = (2_i64.pow(len as u32) - 1) as u64;

    for node in costs {
        for (visited, cost) in node {
            if visited == all_nodes_visited && (candidate.is_none() || compare_func(cost, candidate.unwrap()) == Ordering::Less) {
                candidate = Some(cost)
            }
        }
    }

    candidate.unwrap()
}

fn transform_input(input: &InputType) -> Vec<Vec<Option<i64>>> {
    let mut weights: HashMap<(&str,&str), i64> = HashMap::new();
    for (city0,city1,cost) in input.iter() {
        weights.insert((city0, city1), *cost);
        weights.insert((city1, city0), *cost);
    }

    let mut i = 0;
    let mut names_to_num: HashMap<&str, u64> = HashMap::new();

    for ((city0,city1),_) in weights.iter() {
        if !names_to_num.contains_key(city0) {
            names_to_num.insert(city0, i);
            
            i += 1;
        }

        if !names_to_num.contains_key(city1) {
            names_to_num.insert(city1, i);
            i += 1;
        }
    }

    let mut new_weights: Vec<Vec<Option<i64>>> = vec![vec![None;i as usize];i as usize];

    for ((city0,city1),v) in weights.iter() {
        let i = names_to_num.get(city0).unwrap();
        let j = names_to_num.get(city1).unwrap();
        // unoriented graph
        new_weights[*i as usize][*j as usize] = Some(*v);
        new_weights[*j as usize][*i as usize] = Some(*v);
    }

    new_weights
}

pub fn result_1(input: InputType) -> i64
{
    let weights = transform_input(&input);
    best_path_cost(&weights, |x0: i64, x1: i64| -> Ordering {x0.partial_cmp(&x1).unwrap()})
}


pub fn result_2(input: InputType) -> i64
{  
    let weights = transform_input(&input);
    best_path_cost(&weights, |x0: i64, x1: i64| -> Ordering {x1.partial_cmp(&x0).unwrap()})
}