use std::{fs, vec};

pub type InputType = Vec<i64>;

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];

// returns a list storing the results of (i ^ n) % 10
// for i in 0..10
// fn digits_mod(n: usize) -> Vec<i64> {
//     (0..10)
//         .map(|i| {
//             let mut current = i;
//             let mut seen = vec![i; 1];

//             loop {
//                 current *= i;
//                 current %= 10;

//                 if seen[0] == current {
//                     break;
//                 }

//                 seen.push(current);
//             }

//             return seen[(n - 1) % seen.len()];
//         })
//         .collect::<Vec<i64>>()
// }

fn run(input: &mut InputType, n: usize) {
    let mut buffer = &mut vec![0; input.len()];

    for _ in 0..n {
        for i in input.len() {}

        let temp = input;
        input = buffer;
        buffer = temp;
    }
}

pub fn result_1(mut input: InputType) -> i64 {
    input = run(input, 2);

    input[0..8].into_iter().fold(0, |acc, b| acc * 10 + b)
}

pub fn result_2(input: InputType) -> i64 {
    // // let offset = input[0..7].into_iter().fold(0, |acc, b| acc * 10 + b) as usize;
    // // let length = input.len() * 10000 - offset;
    // let offset = 0;
    // let length = input.len();
    // let mut l = vec![0; length];

    // for i in offset..input.len() {
    //     l[i - offset] = input[i % input.len()];
    // }

    // // this list store the results of (index ^ 10000) % 10
    // let mods = (0..10)
    //     .map(|i| {
    //         let mut current = i;
    //         let mut seen = vec![i; 1];

    //         loop {
    //             current *= i;
    //             current %= 10;

    //             if seen[0] == current {
    //                 break;
    //             }

    //             seen.push(current);
    //         }

    //         return seen[(100 - 1) % seen.len()];
    //     })
    //     .collect::<Vec<i64>>();

    // let mut result = vec![0; l.len()];
    // let mut sums = vec![0; l.len()];
    // let mut current_sum = 0;

    // for i in (0..result.len()).rev() {
    //     result[i] = ((i..result.len())
    //         // todo
    //         .map(|j| mods[l[j] as usize] * BASE_PATTERN[((j + 1) / (i + 1)) % 4])
    //         .sum::<i64>()
    //         % 10)
    //         .abs();
    //     current_sum += result[i];
    //     sums[i] = current_sum;
    // }

    // result[0..8].into_iter().fold(0, |acc, b| acc * 10 + b)
    0
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input[0]
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

// pub fn result_1(mut input: InputType) -> i64 {
//     let mut buffer0 = &mut input;
//     let mut buffer1 = &mut vec![0; buffer0.len()];

//     for _ in 0..100 {
//         for i in 0..buffer0.len() {
//             let mut sum = 0;
//             for j in 0..buffer0.len() {
//                 sum += BASE_PATTERN[((j + 1) / (i + 1)) % 4] * (*buffer0)[j];
//             }
//             (*buffer1)[i] = (sum % 10).abs();
//         }

//         let temp = buffer0;
//         buffer0 = buffer1;
//         buffer1 = temp;
//     }

//     input[0..8].into_iter().fold(0, |acc, b| acc * 10 + b)
// }

// pub fn result_2(input: InputType) -> i64 {
//     let offset = input[0..7].into_iter().fold(0, |acc, b| acc * 10 + b) as usize;
//     let length = input.len() * 10000 - offset;
//     let mut l = vec![0; length];

//     for i in offset..input.len() {
//         l[i - offset] = input[i % input.len()];
//     }

//     let mut buffer0 = &mut l;
//     let mut buffer1 = &mut vec![0; buffer0.len()];

//     for k in 0..100 {
//         println!("{}", k);
//         for i in 0..buffer0.len() {
//             let mut sum = 0;
//             for j in 0..buffer0.len() {
//                 sum += BASE_PATTERN[((j + 1) / (i + 1)) % 4] * (*buffer0)[j];
//             }
//             (*buffer1)[i] = (sum % 10).abs();
//         }

//         let temp = buffer0;
//         buffer0 = buffer1;
//         buffer1 = temp;
//     }

//     l[0..8].into_iter().fold(0, |acc, b| acc * 10 + b)
// }
