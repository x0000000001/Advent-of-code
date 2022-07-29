use std::vec;

// elevator = vec of floors
// floor = vec of objects
// object = (is_microchip, name)
pub type InputType = Vec<Vec<(bool, String)>>;

fn is_path_valid(path: &InputType) -> bool {
    for floor in path {
        for object in floor {
            if object.0 {
                let mut is_fried = false;
                for other_object in floor {
                    // protected if associated generator
                    if !other_object.0 && other_object.1 == object.1 {
                        is_fried = false;
                        break;
                    }
                    // burns if other generator nearby 
                    else if !other_object.0 && other_object.1 != object.1 {
                        is_fried = true;
                    }
                }

                if is_fried {
                    return false;
                }
            }
        }
    }

    true
}

fn is_path_won(path: &InputType) -> bool {
    path[0].is_empty() && path[1].is_empty() && path[2].is_empty()
}

fn get_possible_paths(path: &Vec<Vec<(bool, String)>>, floor: usize) ->  Vec<(InputType, usize)>{
    let mut new_paths: Vec<(InputType, usize)> = vec![];

    let mut adjacent_floors = vec![];
    if floor < 3 { adjacent_floors.push(floor + 1);} 
    if floor > 0 { adjacent_floors.push(floor - 1);}

    let obj_count = path[floor].len();

    for i in 0..(obj_count+1) {
        for j in (i+1)..(obj_count+1) {
            for new_floor in adjacent_floors.iter() {
                let mut new_candidate_path = path.clone();
                let mut to_move_objects = vec![];
                if j != obj_count {
                    to_move_objects.push(new_candidate_path[floor].swap_remove(j));
                }
                to_move_objects.push(new_candidate_path[floor].swap_remove(i));
                
                new_candidate_path[*new_floor].append(&mut to_move_objects);

                if is_path_valid(&new_candidate_path) {
                    new_paths.push((new_candidate_path, *new_floor));
                }
            }
        }
    }

    // println!("INITIAL PATH :");
    // print_path(path, floor);

    // println!("POSSIBILITIES :");
    // for (p,f) in new_paths.iter() {
    //     print_path(p, *f);
    // }
    
    new_paths
}

/// Djisktra's adapted to the situation
pub fn result_1(input: InputType) -> i64
{
    // (elevator, current floor)
    let mut seen_paths: Vec<(InputType, usize)> = vec![];
    // ((elevator, current floor), cost)
    let mut queue: Vec<((InputType, usize), u64)> = vec![((input,0),0);1];

    while let Some(((current_path, current_floor),current_cost)) = queue.pop() {
        // print_path(&current_path, current_floor);
        if is_path_won(&current_path) {
            return current_cost as i64;
        }

        'candidates_loop: for candidate_path in get_possible_paths(&current_path, current_floor) {
            
            if !is_path_valid(&candidate_path.0) {
                continue 'candidates_loop;
            }

            if seen_paths.contains(&candidate_path) {
                continue 'candidates_loop;
            }

            // updating cost of already in queue
            for i in 0..queue.len() {
                if queue[i].0 == candidate_path {
                    queue[i].1 = queue[i].1.min(current_cost+1);
                    continue 'candidates_loop;
                }
            }


            queue.push((candidate_path, current_cost+1));
        }

        seen_paths.push((current_path,current_floor));
        queue.sort_by(|(_, cost0), (_,cost1)| cost1.cmp(cost0));
    }

    0
}


#[allow(dead_code)]
fn print_path(path: &InputType, floor: usize) {
    println!();
    for i in [3,2,1,0] {
        print!("{} : ",i);
        if floor == i {print!("E ")};
        for obj in path[i].iter() {
            print!("{}", if obj.0 {'M'} else {'G'});
            print!("-{} ", obj.1);
        } println!();
    }
    println!();
}

pub fn result_2(_input: InputType) -> i64
{   
    0
}