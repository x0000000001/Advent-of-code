use crate::Solution;

type InputType = u64;

/// I know the resulting operation is a binary rotation to the left,
/// but I can't find why. \
/// (for instance, my input  \
/// 1011011111111011110011 \
/// becomes \
/// 110111111110111100111)
pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut previous_last_valid = input;
    let mut first_valid = 1;
    let mut space_between_valids = 2;

    while first_valid + space_between_valids <= input {
        // println!("{} {} {}", first_valid, space_between_valids, previous_last_valid);
        let temp =
            first_valid + (input - first_valid) / space_between_valids * space_between_valids;
        first_valid = if temp == previous_last_valid {
            first_valid + space_between_valids
        } else {
            first_valid
        };

        space_between_valids *= 2;
        previous_last_valid = temp;
    }

    Solution::from(first_valid as i64)
}

/// This broke my head. \
/// Recursive is not possible so just wrote a for loop,
/// since removing an element is solving problem for n-1
/// with a rotation of 1. \
/// Indices moves are horrible to deal with.
pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut sols: Vec<usize> = vec![0; input as usize];

    sols[0] = 1;

    for i in 1..(input as usize) {
        let removed = (i + 1) / 2 + 1;
        let mut previous_winner = sols[i - 1] + 1;
        if previous_winner == i + 1 {
            previous_winner = 1;
        }

        sols[i] = if previous_winner < removed {
            previous_winner
        } else {
            previous_winner + 1
        };

        if sols[i] == (i + 2) {
            sols[i] = 1;
        }
    }

    Solution::from(*sols.last().unwrap() as i64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>()[0]
        .parse()
        .unwrap()
}
