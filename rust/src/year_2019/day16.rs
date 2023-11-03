use crate::Solution;

type InputType = Vec<i64>;

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];

fn one_pass(input: &Vec<i64>, receiver: &mut Vec<i64>) {
    assert!(receiver.len() == input.len());
    let n = input.len();

    for i in 0..n {
        let mut s = 0;
        for j in i..n {
            s += BASE_PATTERN[((j + 1) / (i + 1)) % 4] * input[j];
        }
        receiver[i] = (s % 10).abs();
    }
}

fn apply_n_passes(input: &mut Vec<i64>, n: usize) {
    let d = input.len();

    let mut b0 = input.clone();
    let mut b1 = vec![0; d];
    let mut b0_ptr = &mut b0;
    let mut b1_ptr = &mut b1;

    for _ in 0..n {
        one_pass(b0_ptr, b1_ptr);
        let temp = b0_ptr;
        b0_ptr = b1_ptr;
        b1_ptr = temp;
    }

    for i in 0..d {
        input[i] = b0_ptr[i];
    }
}

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    apply_n_passes(&mut input, 100);

    Solution::from(input[0..8].into_iter().fold(0, |acc, b| acc * 10 + b))
}

/// SMARTER WAY TO DO THIS \
/// I've not been to the end of this solution on my own,
/// so I've let my incomplete code commented below my final solution.
///
/// It turns out final coefficients are binomial coefficients.
/// They just blow up with the fft iterations count and the size of the input.
/// What we want to do, is calculate these coefficients mod 10.
/// It turns out this isn't trivial, you need [Luca's theorem](https://en.wikipedia.org/wiki/Lucas%27s_theorem).
///
/// Sources : (very elegant soolutions, worth to check) \
/// https://old.reddit.com/r/adventofcode/comments/ebb8w6/2019_day_16_part_three_a_fanfiction_by_askalski/ \
/// https://gist.github.com/alexanderhaupt/1ac31ecbd316aca32c469f42d8646c98
pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let input = input
        .into_iter()
        .map(|x| x as usize)
        .collect::<Vec<usize>>();

    let offset = input[0..7].into_iter().fold(0, |acc, b| acc * 10 + b) as usize;
    let length = input.len() * 10000 - offset;

    let mut l = vec![0; length];

    for i in 0..length {
        l[i] = input[(offset + i) % input.len()];
    }

    for _ in 0..100 {
        let mut s: usize = l.iter().sum();
        for i in 0..length {
            let temp = l[i];
            l[i] = s % 10;
            s -= temp;
        }

        assert!(s == 0);
    }

    Solution::from(l[0..8].into_iter().fold(0, |acc, b| acc * 10 + *b as i64))

    // let input = input
    //     .into_iter()
    //     .map(|x| x as usize)
    //     .collect::<Vec<usize>>();

    // let offset = input[0..7].into_iter().fold(0, |acc, b| acc * 10 + b) as usize;
    // let length = input.len() * 10000 - offset;
    // println!("{} {}", length, input.len());

    // let mut magic_values = vec![1];
    // // let mut magic_values1 = vec![1];

    // for i in 1..length {
    //     magic_values
    //         .push((magic_values[i - 1] * (100 - 1 + i) / i) % (10 * (100 - 1 + i) * (i + 1)));
    //     // magic_values1.push((magic_values[i - 1] * (100 - 1 + i) / i));
    // }

    // // for i in 0..5 {
    // //     for j in 0..5 {
    // //         if j < i {
    // //             print!("  ");
    // //         } else {
    // //             print!("{} ", magic_values1[j - i]);
    // //         }
    // //     }
    // //     println!();
    // // }

    // // for (i, j) in magic_values.iter().zip(magic_values1.iter()) {
    // //     print!("{} {}  |  ", i, j);
    // // }

    // for i in 0..8 {
    //     let mut s = 0;
    //     for j in 0..(length - i) {
    //         s = (s + magic_values[j] * input[(offset + i + j) % input.len()]) % 10;
    //     }

    //     println!("{}", s);
    // }

    // println!();

    // 0
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>()[0]
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}
