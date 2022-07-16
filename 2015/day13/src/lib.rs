use std::{cmp::Ordering, collections::HashMap};

pub type InputType = Vec<(String,String,i64)>;

/// Same djisktra's modified algorithm as 2015::day9. \
/// Difference is : must return to starting node. \
/// To do so, we set only 1 starting point (since paths costs don't depend on a strating point : they are circular) \
/// The trick is to add the starting point a 2nd time in the nodes point a second time
/// and paths can't come back to it if it has not already visited all other nodes. \
/// Right now, trying to fix with giving all start nodes a try (which shoudln't be necessary)
fn best_path_cost(weights: &mut Vec<Vec<Option<i64>>>, compare_func: fn(i64,i64) -> Ordering) -> i64 {
    
        // adding starting point a 2nd time
        let mut new_weights = weights.clone();
        new_weights.push(new_weights[0].clone());
        let len = new_weights.len();

        for i in 0..(len-1) {
            let val = new_weights[0][i].clone();
            new_weights[i].push(val);
        }

        new_weights[len-1].push(None);

        let mut costs: HashMap<u64,i64> = HashMap::new();
        // node, visited
        let mut queue: Vec<(usize, u64)> = vec![];

        let visited = 1; // 2^0
        queue.push((0, visited.clone()));
        costs.insert(visited, 0);

    let all_others_visited = (2_i64.pow((len-1) as u32) - 1) as u64;

        while !queue.is_empty() {
            queue.sort_by(|(_, c0), (_, c1)| compare_func(*c0 as i64,*c1 as i64));
            let (node, visited) = queue.swap_remove(0);
            let cost = costs.get(&visited).unwrap().clone();
            
            for j in 0..len {
                // must check if not already visited
                if (visited >> j)%2 == 1 {
                    continue;
                }

                // can't come back to starting point if not already visited all nodes
                if j == len-1 && visited != all_others_visited {
                    continue;
                }

                if let Some(weight) = new_weights[node][j]{
                    let mut ancient_cost = None;
                    let new_visited = visited + 2_u64.pow(j as u32);
                    if let Some(v) = costs.get(&new_visited) {
                        ancient_cost = Some(*v);
                    }

                    if ancient_cost.is_none() || compare_func(ancient_cost.unwrap(), cost + weight) == Ordering::Greater {
                        let accessor = costs.entry(new_visited).or_insert(0);
                        *accessor = cost + weight;
                        queue.push((j, new_visited));
                    }
                } 
            }
        }

        let mut candidate = None;
        let all_nodes_visited = (2_i64.pow(len as u32) - 1) as u64;

        for (visited, cost) in costs {
            if visited == all_nodes_visited && (candidate.is_none() || compare_func(cost, candidate.unwrap()) == Ordering::Less) {
                candidate = Some(cost)
            }
        }

        candidate.unwrap()
}

// transformation is different from day9, since 2 values add \
/// this function is broken :\
/// * old * sometimes, it returns a valid graph, sometimes not (for the same input)\
/// * old * now, it only returns wrong graph\
/// * old * everything was tested : only this function is incorrect \
/// the graph it returns is correct, the best_path_cost function is broken \
/// idk how but somehow having a graph with different orders give different results,
/// and it gave a good result sometimes \
/// I hate my life
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

    // for l in new_weights.iter() {
    //     for c in l.iter() {
    //         if let Some(d) = c {
    //             print!("{d} ");
    //         }else {
    //             print!("None ")
    //         }
    //     }
    //     println!();
    // }

    new_weights
}

// good answers : 709, 668 (for my input...)

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