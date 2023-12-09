mod helpers;

pub use helpers::Solution;

type InputType = Vec<Vec<i64>>;

fn diffs(seq: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut seqs: Vec<Vec<i64>> = vec![];
    seqs.push(seq.to_owned());
    let mut j = 0;

    loop {
        let mut is_zeros = true;
        seqs.push(vec![0; seqs[j].len() - 1]);

        for i in 0..(seqs[j].len() - 1) {
            let diff = seqs[j][i + 1] - seqs[j][i];
            if diff != 0 {
                is_zeros = false;
            }
            seqs[j + 1][i] = diff;
        }

        if is_zeros {
            break;
        }

        j += 1;
    }

    seqs
}

fn next(seqs: &Vec<Vec<i64>>) -> i64 {
    (0..seqs.len())
        .map(|i| seqs[i].last().unwrap())
        .sum::<i64>()
}

pub fn part1(s: String) -> Solution {
    Solution::from(parse(s).iter().map(diffs).map(|s| next(&s)).sum::<i64>() as u64)
}

fn previous(seqs: &Vec<Vec<i64>>) -> i64 {
    let mut previous_nums = vec![];
    previous_nums.push(0);

    for i in (0..seqs.len() - 1).rev() {
        previous_nums.push(seqs[i].first().unwrap() - previous_nums.last().unwrap());
    }

    *previous_nums.last().unwrap()
}

pub fn part2(s: String) -> Solution {
    Solution::from(
        parse(s)
            .iter()
            .map(diffs)
            .map(|s| previous(&s))
            .sum::<i64>() as u64,
    )
}

fn parse(s: String) -> InputType {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}
