use std::{collections::HashMap, fs};

pub type InputType = Vec<i128>;

fn shuffle(
    indexes: &mut HashMap<usize, usize>,
    vals: &mut HashMap<usize, usize>,
    input: &InputType,
) {
    let n = input.len();
    let ni128 = n as i128;

    for i in 0..n {
        let index = *indexes.get(&i).unwrap();
        let temp = index as i128 + input[i];

        let new_index = if temp >= ni128 {
            (temp + 1) % ni128
        } else if temp < 0 {
            (ni128 * 1000000000000000 + temp - 1) % ni128
        } else {
            temp
        } as usize;

        if new_index > index {
            for j in (index + 1)..(new_index + 1) {
                let v = *vals.get(&j).unwrap();
                indexes.insert(v, j - 1);
                vals.insert(j - 1, v);
            }
        } else if new_index < index {
            for j in (new_index..index).rev() {
                let v = *vals.get(&j).unwrap();
                indexes.insert(v, j + 1);
                vals.insert(j + 1, v);
            }
        }

        indexes.insert(i, new_index);
        vals.insert(new_index, i);
    }
}

pub fn result_1(input: InputType) -> i64 {
    let n = input.len();
    let mut indexes: HashMap<usize, usize> = (0..n).map(|i| (i, i)).collect();
    let mut vals: HashMap<usize, usize> = indexes.clone();
    let index_of_0_temp = input
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == 0)
        .next()
        .unwrap()
        .0;

    shuffle(&mut indexes, &mut vals, &input);

    let index_of_0 = indexes.get(&index_of_0_temp).unwrap();

    (input[*vals.get(&((index_of_0 + 1000) % n)).unwrap()]
        + input[*vals.get(&((index_of_0 + 2000) % n)).unwrap()]
        + input[*vals.get(&((index_of_0 + 3000) % n)).unwrap()]) as i64
}

pub fn result_2(input: InputType) -> i64 {
    let n = input.len();
    let mut modified_input = input.clone();

    let mut indexes: HashMap<usize, usize> = (0..n).map(|i| (i, i)).collect();
    let mut vals: HashMap<usize, usize> = indexes.clone();
    let index_of_0_temp = input
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == 0)
        .next()
        .unwrap()
        .0;

    for k in 0..n {
        modified_input[k] = (input[k] * 811589153) % (n as i128 - 1);
    }

    for _ in 0..10 {
        shuffle(&mut indexes, &mut vals, &modified_input);

        // for k in 0..n {
        //     print!("{} ", input[*vals.get(&k).unwrap()] * 811589153);
        // }
        // println!();
    }

    let index_of_0 = indexes.get(&index_of_0_temp).unwrap();

    (input[*vals.get(&((index_of_0 + 1000) % n)).unwrap()] * 811589153
        + input[*vals.get(&((index_of_0 + 2000) % n)).unwrap()] * 811589153
        + input[*vals.get(&((index_of_0 + 3000) % n)).unwrap()] * 811589153) as i64
}

// -2676621026594
pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect();

    input.into_iter().map(|l| l.parse().unwrap()).collect()
}
