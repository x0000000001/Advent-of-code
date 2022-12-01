use std::vec;

// elevator = vec of floors
// floor = vec of objects
// object = (is_microchip, name)
// (elevator,current_floor)
pub type InputType = (Vec<Vec<(bool, u64)>>, usize);

fn is_path_valid((elev,_): &InputType) -> bool {
    for floor in elev {
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

fn is_path_won((elev,_): &InputType) -> bool {
    elev[0].is_empty() && elev[1].is_empty() && elev[2].is_empty()
}

/// Returns the list of pairs of floors to describe an elevator. \
/// For instance, a microchip on floor 3 with its corresponding
/// generator on floor 1 would give (3,1). \
/// The list is then sorted.
fn floors_pairs(elev: &Vec<Vec<(bool,u64)>>) -> Vec<(usize,usize)>{
    let mut pairs = vec![];

    for i in 0..4 {
        'object_loop: for (is_m,id) in elev[i].iter() {
            if !is_m {
                continue;
            }

            for j in 0..4 {
                for (is_m1,id1) in elev[j].iter(){
                    if !is_m1 && id1 == id {
                        pairs.push((i,j));
                        continue 'object_loop;
                    }
                }
            }
        }
    }

    pairs.sort_unstable();
    pairs
}

fn get_possible_paths((elev,floor): &InputType) ->  Vec<InputType>{
    let mut new_paths: Vec<InputType> = vec![];

    let floor = *floor;

    let mut adjacent_floors = vec![];
    if floor < 3 { adjacent_floors.push(floor + 1);} 

    // useless to bring objects down when there is nothing under the current floor
    let mut are_bottom_floors_empty = true;

    for i in 0..floor {
        if !elev[i].is_empty() {
            are_bottom_floors_empty = false;
            break;
        }
    }

    if !are_bottom_floors_empty && floor > 0 { adjacent_floors.push(floor - 1);}

    let obj_count = elev[floor].len();

    for i in 0..(obj_count+1) {
        for j in (i+1)..(obj_count+1) {
            for new_floor in adjacent_floors.iter() {
                let mut new_elev = elev.clone();
                let mut to_move_objects = vec![];
                if j != obj_count {
                    to_move_objects.push(new_elev[floor].swap_remove(j));
                }
                to_move_objects.push(new_elev[floor].swap_remove(i));
                
                new_elev[*new_floor].append(&mut to_move_objects);

                let new_candidate_path = (new_elev,*new_floor);

                if is_path_valid(&new_candidate_path) {
                    new_paths.push(new_candidate_path);
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

fn djisktra_elevator(input: InputType) -> i64 {
    // (inputtype,foors_pairs)
    let mut seen_paths: Vec<(InputType,Vec<(usize,usize)>)> = vec![];
    // (inputtype, cost, floors_pairs)
    let input_pairs = floors_pairs(&input.0);
    let mut queue: Vec<(InputType, u64, Vec<(usize,usize)>)> = vec![(input,0,input_pairs);1];

    while let Some((current_path,current_cost,current_pairs)) = queue.pop() {
        // print_path(&current_path);
        // println!("{:?}",get_floors_pairs(&current_path.0));
        if is_path_won(&current_path) {
            return current_cost as i64;
        }

        'candidates_loop: for candidate_path in get_possible_paths(&current_path) {

            let candidate_pairs = floors_pairs(&candidate_path.0);

            for p_seen in seen_paths.iter() {
                if p_seen.0.1 == candidate_path.1 && candidate_pairs == p_seen.1 {
                    continue 'candidates_loop;
                }
            }

            // updating cost of already in queue
            for i in 0..queue.len() {
                if queue[i].0.1 == candidate_path.1 && queue[i].2 == candidate_pairs {
                    queue[i].1 = queue[i].1.min(current_cost+1);
                    continue 'candidates_loop;
                }
            }


            queue.push((candidate_path, current_cost+1, candidate_pairs));
        }

        seen_paths.push((current_path, current_pairs));
        queue.sort_by(|(_, cost0, _), (_,cost1, _)| cost1.cmp(cost0));
    }

    0
}

/// Djisktra's adapted to the situation
pub fn result_1(input: InputType) -> i64
{
    djisktra_elevator(input)
}


#[allow(dead_code)]
fn print_path((elev,floor): &InputType) {
    println!();
    for i in [3,2,1,0] {
        if *floor == i {
            print!("E {} : ",i);
        } else {
            print!("  {} : ",i);
        }
        for obj in elev[i].iter() {
            print!("{}", if obj.0 {'M'} else {'G'});
            print!("-{} ", obj.1);
        } println!();
    }
    println!();
}

/// This one takes one minute to run. \
/// Given where I came from, this seems acceptable to me, 
/// knowing this problem is insanely hard for a day11 problem.
pub fn result_2(mut input: InputType) -> i64
{   
    input.0[0].push((false,100));
    input.0[0].push((true,100));
    input.0[0].push((false,101));
    input.0[0].push((true,101));
    djisktra_elevator(input)
}