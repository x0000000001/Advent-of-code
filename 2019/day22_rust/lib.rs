use std::{collections::HashMap, fs};

pub type InputType = Vec<Shuffle>;

#[derive(Clone, Copy)]
pub enum Shuffle {
    Deal(usize),
    Cut(i64),
    Stack,
}

pub fn result_1(input: InputType) -> i64 {
    let mut position = 2019;
    let count = 10007;

    for s in input {
        match s {
            Shuffle::Deal(n) => position = (position * n) % count,
            Shuffle::Cut(x) => {
                let n = x.abs() as usize;
                if x > 0 {
                    if position < n {
                        position += count - n;
                    } else {
                        position -= n;
                    }
                } else {
                    if position > count - 1 - n {
                        position -= count - n;
                    } else {
                        position += n;
                    }
                }
            }
            Shuffle::Stack => position = count - 1 - position,
        }
    }

    position as i64
}

pub fn result_2(mut input: InputType) -> i64 {
    input.reverse();
    let mut position = 2020;
    let count = 119315717514047;
    let mut seen: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;
    let steps = 101741582076661 * input.len();

    while i < steps {
        if i % input.len() == 0 {
            if let Some(index) = seen.get(&position) {
                let cycle_length = i - index;
                let repeat_times = (steps - 1 - i) / cycle_length;
                if repeat_times > 0 {
                    i += repeat_times * cycle_length;
                    continue;
                }
            }

            seen.insert(position, i);
        }

        let s = input[i % input.len()];

        match s {
            Shuffle::Deal(n) => {
                // p = n
                // q = count
                // since p and q are primes together,
                // (p ^ (q-1)) % q = 1
                // and we are looking for x so that (p * x) % q = n
                // this https://www.maths.ox.ac.uk/system/files/attachments/lecture2.pdf
                // tells us that if (a * b) % c = 1, then dividing by a is the same as multiplying by b in Z / cZ (cyclic group)
                // so we can divide the left hand by p by multiplying by p ^ (q - 2)
                // I am very proud of having found this alone (this must be somehow related to fermat's theorem right ?)
                let mut pow = 1;

                for _ in 0..(count - 2) {
                    pow *= n;
                    pow %= count;
                }

                position = (pow * position) % count;
            }
            Shuffle::Cut(mut x) => {
                x *= -1;
                let n = x.abs() as usize;
                if x > 0 {
                    if position < n {
                        position += count - n;
                    } else {
                        position -= n;
                    }
                } else {
                    if position > count - 1 - n {
                        position -= count - n;
                    } else {
                        position += n;
                    }
                }
            }
            Shuffle::Stack => position = count - 1 - position,
        }

        i += 1;

        println!("{}", position);
    }

    position as i64
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
