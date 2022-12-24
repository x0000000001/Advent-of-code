use std::fs;

pub type InputType = Vec<Shuffle>;

#[derive(Clone, Copy)]
pub enum Shuffle {
    Deal(i128),
    Cut(i128),
    Stack,
}

type Matrix = Vec<Vec<i128>>;

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

pub fn result_1(input: InputType) -> i64 {
    let mut position = 2019;
    let count = 10007;

    for s in input {
        match s {
            Shuffle::Deal(n) => position = (position * n) % count,
            Shuffle::Cut(x) => position = (position + count - x) % count,
            Shuffle::Stack => position = count - 1 - position,
        }
    }

    position as i64
}

/// https://fr.wikipedia.org/wiki/Exponentiation_modulaire
fn modular_exp(mut p: i128, mut exp: i128, modulo: i128) -> i128 {
    let mut res = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            res *= p;
            res %= modulo;
        }

        exp /= 2;
        p *= p;
        p %= modulo;
    }

    res
}

fn reversal_matrix(input: &InputType, count: i128) -> Matrix {
    let mut m: Matrix = Vec::from([Vec::from([1, 0]), Vec::from([0, 1])]);

    for instr in input.iter().rev() {
        match instr {
            Shuffle::Deal(n) => {
                m = matrix_mul(
                    &Vec::from([
                        Vec::from([modular_exp(*n, count - 2, count), 0]),
                        Vec::from([0, 1]),
                    ]),
                    &m,
                )
            }
            Shuffle::Cut(x) => {
                m = matrix_mul(
                    &Vec::from([Vec::from([1, count + x]), Vec::from([0, 1])]),
                    &m,
                )
            }
            Shuffle::Stack => {
                m = matrix_mul(
                    &Vec::from([Vec::from([-1, count - 1]), Vec::from([0, 1])]),
                    &m,
                )
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                m[i][j] += 100000 * count;
                m[i][j] %= count;
            }
        }
    }

    m
}

fn matrix_fast_exp(m: &Matrix, n: usize, modulo: i128) -> Matrix {
    if n == 0 {
        return Vec::from([Vec::from([1, 0]), Vec::from([0, 1])]);
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

    res
}

/// Method : \
/// 1 - Representing a linear operation as a 2x2 matrix ```[[a b] [0 1]]``` applied to ```[[x] [1]]``` \
/// 2 - Finding the operation to reverse the "deal" operation -> this is named [modulo inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse) \
/// 3 - Computing the matrix to reverse a round of shuffles by multiplying all reverse matrix in reverse order. \
/// 4 - Fast exponentiating this matrix to the ridiculous amount of shuffle rounds needed. \
/// \
/// To avoid overflow, using modulo basically everywhere and using i128. \
/// This. was. hard. (at my personnal level)
pub fn result_2(input: InputType) -> i64 {
    let mut m = Vec::from([Vec::from([2020]), Vec::from([1])]);
    let count = 119315717514047;
    let steps = 101741582076661;

    // FOR TEST PURPOSES
    // let mut m = Vec::from([Vec::from([2519]), Vec::from([1])]);
    // let count = 10007;
    // let steps = 1;

    let mut rev_m = reversal_matrix(&input, count);
    rev_m = matrix_fast_exp(&rev_m, steps, count);

    m = matrix_mul(&rev_m, &m);

    for i in 0..2 {
        for j in 0..1 {
            m[i][j] %= count;
        }
    }

    m[0][0] as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    let get_shuffle = |l: &str| -> Shuffle {
        let w = l.split_whitespace().collect::<Vec<&str>>();

        match w[0] {
            "cut" => Shuffle::Cut(w[1].parse().unwrap()),
            _ => match w[1] {
                "with" => Shuffle::Deal(w[3].parse().unwrap()),
                _ => Shuffle::Stack,
            },
        }
    };

    input.into_iter().map(|l| get_shuffle(&l)).collect()
}
