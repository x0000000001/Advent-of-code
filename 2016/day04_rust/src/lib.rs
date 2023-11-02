use std::{cmp::Ordering, collections::HashMap};

pub type InputType = Vec<(Vec<String>, Vec<char>, i64)>;

pub fn is_valid((names, letters, _): &(Vec<String>, Vec<char>, i64)) -> bool {
    let mut counts: HashMap<char, i64> = HashMap::new();

    for n in names {
        for c in n.chars() {
            let accessor = counts.entry(c).or_insert(0);
            *accessor += 1;
        }
    }

    let mut temp = counts.into_iter().collect::<Vec<(char, i64)>>();
    temp.sort_by(|(char0, c0), (char1, c1)| {
        let res = c1.cmp(c0);
        if res == Ordering::Equal {
            char0.cmp(char1)
        } else {
            res
        }
    });
    let most_recurrent_letters = temp.into_iter().map(|(c, _)| c).collect::<Vec<char>>();
    most_recurrent_letters[0..5] == *letters
}

pub fn result_1(input: InputType) -> i64 {
    input
        .into_iter()
        .filter(|el| is_valid(&el))
        .map(|(_, _, c)| c)
        .sum::<i64>()
}

pub fn result_2(input: InputType) -> i64 {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for (words, _, n) in input.into_iter().filter(|el| is_valid(el)) {
        let chars: Vec<char> = words
            .into_iter()
            .flat_map(|el| el.chars().collect::<Vec<char>>())
            .collect();

        let indices: Vec<usize> = chars
            .iter()
            .map(|el| alphabet.iter().position(|l| l == el).unwrap())
            .collect();
        let indices: Vec<usize> = indices.into_iter().map(|p| (p + n as usize) % 26).collect();

        let new_chars: String = indices.into_iter().map(|el| alphabet[el]).collect();
        if new_chars.contains("northpole") {
            return n;
        }
    }
    0
}
