use std::{fs, vec};

pub type InputType = Vec<i64>;

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];

type Matrix = Vec<Vec<i64>>;

fn matrix_mul(m0: &Matrix, m1: &Matrix) -> Matrix {
    assert!(m0[0].len() == m1.len());
    let n = m1.len();
    let h = m0.len();
    let w = m1[0].len();

    (0..h)
        .map(|i| {
            (0..w)
                .map(|j| (0..n).map(|k| m0[i][k] * m1[k][j]).sum())
                .collect()
        })
        .collect()
}

fn matrix_fast_exp(m: &Matrix, n: usize, modulo: i64) -> Matrix {
    if n == 0 {
        let mut mid = vec![vec![0; m.len()]; m.len()];
        for i in 0..m.len() {
            mid[i][i] = 1;
        }
        return mid;
    }

    let temp_mat = matrix_fast_exp(m, n / 2, modulo);

    let mut res = matrix_mul(&temp_mat, &temp_mat);

    for i in 0..res.len() {
        for j in 0..m[0].len() {
            res[i][j] %= modulo;
        }
    }

    if n % 2 == 1 {
        res = matrix_mul(&res, m);
    }

    for i in 0..res.len() {
        for j in 0..m[0].len() {
            res[i][j] %= modulo;
        }
    }

    print!("");

    res
}

fn pattern_matrix(length: usize) -> Matrix {
    let mut m = vec![vec![0; length]; length];

    for i in 0..length {
        for j in i..length {
            m[i][j] = BASE_PATTERN[((j + 1) / (i + 1)) % 4];
        }
    }

    m
}

fn iter_fft(input: &Matrix, steps: usize) -> Matrix {
    let length = input.len();
    let mut pattern_m = pattern_matrix(length);
    pattern_m = matrix_fast_exp(&pattern_m, steps, 10);

    matrix_mul(&pattern_m, input)
}

pub fn result_1(input: InputType) -> i64 {
    let mut output: Matrix = input.into_iter().map(|e| Vec::from([e])).collect();
    output = iter_fft(&output, 100);
    output[0..8].into_iter().fold(0, |acc, b| acc * 10 + b[0])
}

pub fn result_2(input: InputType) -> i64 {
    let offset = input[0..7].into_iter().fold(0, |acc, b| acc * 10 + b) as usize;
    let length = input.len() * 10000 - offset;
    let mut l = vec![0; length];

    for i in offset..input.len() {
        l[i - offset] = input[i % input.len()];
    }

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

// fn run(input: &mut InputType, n: usize) {
//     let mut buffer = &mut vec![0; input.len()];

//     for _ in 0..n {
//         for i in input.len() {}

//         let temp = input;
//         input = buffer;
//         buffer = temp;
//     }
// }

// pub fn result_2(input: InputType) -> i64 {
//     // let offset = input[0..7].into_iter().fold(0, |acc, b| acc * 10 + b) as usize;
//     // let length = input.len() * 10000 - offset;
//     let offset = 0;
//     let length = input.len();
//     let mut l = vec![0; length];

//     for i in offset..input.len() {
//         l[i - offset] = input[i % input.len()];
//     }

//     // this list store the results of (index ^ 10000) % 10
//     let mods = (0..10)
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

//             return seen[(100 - 1) % seen.len()];
//         })
//         .collect::<Vec<i64>>();

//     let mut result = vec![0; l.len()];
//     let mut sums = vec![0; l.len()];
//     let mut current_sum = 0;

//     for i in (0..result.len()).rev() {
//         result[i] = ((i..result.len())
//             // todo
//             .map(|j| mods[l[j] as usize] * BASE_PATTERN[((j + 1) / (i + 1)) % 4])
//             .sum::<i64>()
//             % 10)
//             .abs();
//         current_sum += result[i];
//         sums[i] = current_sum;
//     }

//     result[0..8].into_iter().fold(0, |acc, b| acc * 10 + b)
//     0
// }
