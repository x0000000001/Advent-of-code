use std::vec;

pub type InputType = Vec<u64>;

// next groupe of n distincts elements from a subset of max + 1 elements
fn next_group(comb: &mut Vec<usize>, max: usize, index: usize) -> bool {
    if index == comb.len() - 1 {
        if comb[index] < max {
            comb[index] += 1;
        } else {
            return false;
        }
    } else {
        if comb[index] + 1 < comb[index+1] {
            comb[index] += 1;
        } else {
            for i in 0..(index+1) {
                comb[i] = i;
            }
            if !next_group(comb, max, index+1) {
                return false;
            }
        }
    }

    true
}

// next number following comb in a max-radix
fn next(comb: &mut Vec<usize>, max: usize, index: usize) -> bool {
    if index == comb.len() - 1 && comb[index] == max {
        false
    } else if comb[index] < max {
        comb[index] += 1;
        true
    } else {
        comb[index] = 0;
        next(comb, max,index+1)
    }
}

fn are_there_n_equal_groups(packages: &Vec<u64>, n: usize) -> bool {
    let mut comb = vec![0;packages.len()];
    let goal = packages.iter().sum::<u64>() / n as u64;

    'main: loop {
        let mut sums = vec![0;n];
        // rev is necessary for optimization to work
        for i in (0..packages.len()).rev(){
            sums[comb[i]] += packages[i];
            // optimization
            if sums[comb[i]] > goal {
                if !next(&mut comb, n-1, i) {
                    break 'main;
                }
                continue 'main;
            }
            
        }

        let mut are_equal = true;

        for i in 0..n {
            if sums[i] != goal {
                are_equal = false;
                break;
            }
        }

        if are_equal {
            return true;
        }

        if !next(&mut comb, n-1, 0) {
            break;
        }
    }

    false
}

fn find_optimized_entanglement(packages: InputType, groups: usize) -> u64 {
    let len = packages.len();
    let goal: u64 = packages.iter().sum::<u64>()/groups as u64;

    for i in 1..(packages.len()-2) {
        let mut comb: Vec<usize> = (0..i).collect();
        let mut min_qe = None;
        loop {
            let sum: u64 = comb.iter().map(|el| packages[*el]).sum();
            
            if sum == goal {
                let mut others = packages.clone();
                for c in comb.iter().rev() {
                    others.swap_remove(*c);
                }

                if are_there_n_equal_groups(&others, groups-1) {
                    min_qe = Some(comb.iter().map(|el| packages[*el]).product::<u64>());
                }
            }

            
            if !next_group(&mut comb, len-1, 0) {
                break;
            }
        }

        if let Some(qe) = min_qe {
            return qe;
        }
    }

    0
}

/// testing groups by groups length
pub fn result_1(packages: InputType) -> i64 {
    find_optimized_entanglement(packages, 3) as i64
}

/// Execution time : 37s. \
/// From where I came from beginning this problem, I feel satisfied.
pub fn result_2(packages: InputType) -> i64 {
    find_optimized_entanglement(packages, 4) as i64
}














































// first attempts

// fn next(comb: &mut Vec<usize>, index: usize) -> bool {
//     if index < 5 {
//         println!("{:?}", comb);
//     }
//     if index == 0 && comb[0] == 2 {
//         false
//     } else if comb[index] < 2 {
//         comb[index] += 1;
//         true
//     } else {
//         comb[index] = 0;
//         next(comb, index-1)
//     }
// }

// /// Did it take 334.33s to run ? Yep.
// pub fn result_1(packages: InputType) -> i64
// {
//     let mut min_count = usize::MAX;
//     let mut min_qe = u64::MAX;
//     let goal: u64 = packages.iter().sum::<u64>()/3;
//     let len = packages.len();
//     let mut comb = vec![0;len];

//     'main: loop {
//         let mut sums = [0,0,0];
//         let mut counts = [0,0,0];

        
//         for i in 0..len {
//             sums[comb[i]] += packages[i];
//             counts[comb[i]] += 1;

//             if sums[comb[i]] > goal || counts[comb[i]] > min_count {
//                 if !next(&mut comb, i) {
//                     break 'main;
//                 }
//                 continue 'main;
//             }
//         }

//         min_count = *counts.iter().min().unwrap();
//         min_qe = u64::MAX;

//         for i in 0..3 {
//             if counts[i] == min_count {
//                 let candidate_min_qe: u64 = packages.iter().enumerate().filter(|(index, _)| comb[*index] == i).map(|(_,val)| val).product();
//                 if candidate_min_qe < min_qe {
//                     min_qe = candidate_min_qe;
//                     println!("{min_qe} {:?}", comb);
//                 }
//             }
//         }

//         if !next(&mut comb, len - 1) {
//             break 'main;
//         }
//     }

//     min_qe as i64
// }

// fn next2(comb: &mut Vec<usize>, index: usize) -> bool {
//     if index < 5 {
//         println!("{:?}", comb);
//     }
//     if index == 0 && comb[0] == 3 {
//         false
//     } else if comb[index] < 3 {
//         comb[index] += 1;
//         true
//     } else {
//         comb[index] = 0;
//         next2(comb, index-1)
//     }
// }

// pub fn result_2(packages: InputType) -> i64
// {   
//     let mut min_count = usize::MAX;
//     let mut min_qe = u64::MAX;
//     let goal: u64 = packages.iter().sum::<u64>()/4;
//     let len = packages.len();
//     let mut comb = vec![0;len];

//     'main: loop {
//         let mut sums = [0,0,0,0];
//         let mut counts = [0,0,0,0];

        
//         for i in 0..len {
//             sums[comb[i]] += packages[i];
//             counts[comb[i]] += 1;

//             if sums[comb[i]] > goal || counts[comb[i]] > min_count {
//                 if !next2(&mut comb, i) {
//                     break 'main;
//                 }
//                 continue 'main;
//             }
//         }

//         min_count = *counts.iter().min().unwrap();
//         min_qe = u64::MAX;

//         for i in 0..4 {
//             if counts[i] == min_count {
//                 let candidate_min_qe: u64 = packages.iter().enumerate().filter(|(index, _)| comb[*index] == i).map(|(_,val)| val).product();
//                 if candidate_min_qe < min_qe {
//                     min_qe = candidate_min_qe;
//                     println!("{min_qe} {:?}", comb);
//                 }
//             }
//         }

//         if !next2(&mut comb, len - 1) {
//             break 'main;
//         }
//     }

//     min_qe as i64
// }