use std::fs;

pub type InputType = (Vec<Vec<Node>>,(usize,usize));

#[derive(Clone, Copy, PartialEq)]
pub enum Node {
    Free,
    Occupied,
    ToGo
}

// length of shortest path between two points
fn djikstra(start: &(usize,usize), end: &(usize,usize), grid: &Vec<Vec<Node>>) -> u64 {
    let (w,h) = (grid.len(),grid[0].len());
    let mut seen: Vec<(usize,usize)> = vec![];
    let mut queue: Vec<((usize,usize),u64)> = vec![];
    queue.push((*start,0));

    while !queue.is_empty() {
        queue.sort_by_key(|(_,c)| *c);
        let (node,cost) = queue.swap_remove(0);

        if node == *end {
            return cost;
        }

        let mut neighbors = vec![];
        if node.0 > 0 { neighbors.push((node.0-1,node.1))};
        if node.1 > 0 { neighbors.push((node.0,node.1-1))};
        if node.0 + 1 < w { neighbors.push((node.0+1,node.1))};
        if node.1 + 1 < h { neighbors.push((node.0,node.1+1))};

        'candidates_loop: for n in neighbors {
            if grid[n.0][n.1] == Node::Occupied {
                continue;
            }

            if seen.contains(&n) {
                continue;
            }

            for i in 0..queue.len() {
                if queue[i].0 == n {
                    queue[i].1 = queue[i].1.min(cost+1);
                    continue 'candidates_loop;
                }
            }

            queue.push((n,cost+1));
        }

        seen.push(node);
    }

    0
}

fn modified_djikstra(start: usize, weights: &Vec<Vec<u64>>) -> u64 {
    // to keep track of visited nodes, using representation of visited with an u64 x : 
    // node i has been visited, then ith bit of x is 1
    let mut seen: Vec<(usize,u64)> = vec![];
    let mut queue: Vec<(usize,u64,u64)> = vec![];
    
    queue.push((start,2_u64.pow(start as u32),0));
    
    let mut all_visited = 0;
    for i in 0..weights.len() {
        all_visited += 2_u64.pow(i as u32);
    }

    while !queue.is_empty() {
        queue.sort_by_key(|(_,_,c)| *c);
        let (node,visited,cost) = queue.swap_remove(0);

        if visited == all_visited {
            return cost;
        }

        'candidates_loop: for i in 0..weights.len() {
            let mut test_visited = visited;
            for _ in 0..i {
                test_visited /= 2;
            }

            if test_visited%2 == 1 {
                continue;
            }

            let new_visited = visited + 2_u64.pow(i as u32);
            let w = weights[node][i];

            if seen.contains(&(i,new_visited)) {
                continue;
            }

            for j in 0..queue.len() {
                if queue[j].0 == i && queue[j].1 == new_visited  {  
                    queue[j].2 = queue[j].2.min(cost + w);
                    continue 'candidates_loop;
                }
            }

            queue.push((i,new_visited,cost + w));
        }

        seen.push((node,visited));
    }

    0
}

/// First step : weights between all paths. \
/// Second step : modified Djisktra on found weights.
pub fn result_1((grid,start): InputType) -> i64
{
    let to_go = grid.iter().enumerate().flat_map(
        |(x,line)| line.iter().enumerate()
            .filter(|(_,node)| **node == Node::ToGo).map(
                move |(y,_)| (x,y)
            )
    ).collect::<Vec<(usize,usize)>>();

    let mut weights = vec![vec![0;to_go.len()];to_go.len()];

    for i in 0..to_go.len() {
        for j in 0..i {
            weights[i][j] = djikstra(&to_go[i], &to_go[j], &grid);
            weights[j][i] = weights[i][j];
        }
    }

    let mut start_id = 0;

    for n in to_go {
        if n == start {
            break;
        }

        start_id += 1;
    }

    modified_djikstra(start_id, &weights) as i64
}

fn modified_djikstra2(start: usize, weights: &Vec<Vec<u64>>) -> u64 {
    // to keep track of visited nodes, using representation of visited with an u64 x : 
    // node i has been visited, then ith bit of x is 1
    let mut seen: Vec<(usize,u64)> = vec![];
    let mut queue: Vec<(usize,u64,u64)> = vec![];
    
    queue.push((start,2_u64.pow(start as u32),0));
    
    let mut all_visited = 0;
    for i in 0..weights.len() {
        all_visited += 2_u64.pow(i as u32);
    }

    'main_loop: while !queue.is_empty() {
        queue.sort_by_key(|(_,_,c)| *c);
        let (node,visited,cost) = queue.swap_remove(0);

        if visited == all_visited + 1 {
            return cost;
        }

        if visited == all_visited {
            let w = weights[node][start];
            for j in 0..queue.len() {
                if queue[j].0 == node && queue[j].1 == all_visited+1  {  
                    queue[j].2 = queue[j].2.min(cost + w);
                    continue 'main_loop;
                }
            }

            queue.push((node,all_visited+1,cost+w));
        }

        'candidates_loop: for i in 0..weights.len() {
            let mut test_visited = visited;
            for _ in 0..i {
                test_visited /= 2;
            }

            if test_visited%2 == 1 {
                continue;
            }

            let new_visited = visited + 2_u64.pow(i as u32);
            let w = weights[node][i];

            if seen.contains(&(i,new_visited)) {
                continue;
            }

            for j in 0..queue.len() {
                if queue[j].0 == i && queue[j].1 == new_visited  {  
                    queue[j].2 = queue[j].2.min(cost + w);
                    continue 'candidates_loop;
                }
            }

            queue.push((i,new_visited,cost + w));
        }

        seen.push((node,visited));
    }

    0
}

pub fn result_2((grid,start): InputType) -> i64
{   
    let to_go = grid.iter().enumerate().flat_map(
        |(x,line)| line.iter().enumerate()
            .filter(|(_,node)| **node == Node::ToGo).map(
                move |(y,_)| (x,y)
            )
    ).collect::<Vec<(usize,usize)>>();

    let mut start_id = 0;

    for n in to_go.iter() {
        if *n == start {
            break;
        }

        start_id += 1;
    }

    let mut weights = vec![vec![0;to_go.len()];to_go.len()];

    for i in 0..to_go.len() {
        for j in 0..i {
            weights[i][j] = djikstra(&to_go[i], &to_go[j], &grid);
            weights[j][i] = weights[i][j];
        }
    }

    modified_djikstra2(start_id, &weights) as i64
}

pub fn read_input(path: &str) -> InputType
{
    let contents= fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let w = input.len();
    let h = input[0].len();

    let mut res: Vec<Vec<Node>> = vec![vec![Node::Occupied;h];w];
    let mut start = (0,0);

    let mut x = 0;
    for l in input {
        let mut y = 0;
        for c in l.chars() {
            res[x][y] = match c {
                '#' => Node::Occupied,
                '.' => Node::Free,
                _ => Node::ToGo
            };

            if c == '0' {
                start = (x,y);
            }

            y += 1;
        }
        x += 1;
    }

    (res,start)
}

#[allow(dead_code)]
const TEST_INPUT_PATH: &str = "test_input.txt";

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn test1()
    {
        assert_eq!(result_1(read_input(TEST_INPUT_PATH)), 14);
    }
}