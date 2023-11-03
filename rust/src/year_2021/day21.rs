use std::collections::HashMap;

use crate::Solution;

type InputType = (u64, u64);

pub fn part1(s: String) -> Solution {
    let (mut pawn0, mut pawn1) = parse(s);
    let (mut score0, mut score1) = (0, 0);
    let mut dice = 1;
    let mut rolled_count = 0;

    let mut roll_dice = |pawn: &mut u64, total: &mut u64| {
        let mut score: u64 = dice;
        dice = dice % 100 + 1;
        score += dice;
        dice = dice % 100 + 1;
        score += dice;
        dice = dice % 100 + 1;

        rolled_count += 3;

        *pawn = ((*pawn + score) - 1) % 10 + 1;

        *total += *pawn;
    };

    loop {
        roll_dice(&mut pawn0, &mut score0);
        if score0 >= 1000 {
            break;
        };
        roll_dice(&mut pawn1, &mut score1);
        if score1 >= 1000 {
            break;
        };
    }

    let looser_score = if score0 >= 1000 { score1 } else { score0 };

    Solution::from(looser_score as i64 * rolled_count as i64)
}

const OUTCOMES_REPARTITION: &'static [(u64, u64)] =
    &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn calculate_occ(
    pawn0: u64,
    pawn1: u64,
    total0: u64,
    total1: u64,
    occ: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>,
) {
    let (mut won0, mut won1) = (0, 0);

    for (score0, rep0) in OUTCOMES_REPARTITION.iter() {
        // player 0 wins (leaf)
        let new_pawn0 = (pawn0 + score0 - 1) % 10 + 1;
        let new_total0 = total0 + new_pawn0;

        if new_total0 > 20 {
            won0 += rep0;
            continue;
        } else {
            for (score1, rep1) in OUTCOMES_REPARTITION.iter() {
                // player 1 wins (leaf)
                let new_pawn1 = (pawn1 + score1 - 1) % 10 + 1;
                let new_total1 = total1 + new_pawn1;

                if new_total1 > 20 {
                    won1 += rep0 * rep1;
                    continue;
                }

                let new_key = &(new_pawn0, new_pawn1, new_total0, new_total1);

                // recursing if entry not already in dictionnary
                if !occ.contains_key(new_key) {
                    calculate_occ(new_pawn0, new_pawn1, new_total0, new_total1, occ);
                }

                let (temp0, temp1) = occ.get(new_key).unwrap();

                won0 += temp0 * rep0 * rep1;
                won1 += temp1 * rep0 * rep1;
            }
        }
    }

    occ.insert((pawn0, pawn1, total0, total1), (won0, won1));
}

pub fn part2(s: String) -> Solution {
    let (pawn0, pawn1) = parse(s);

    // dictionnary storing situations already calculated
    // key : (pawn0, pawn1, score0, score1)
    // value : (count of universes where 0 wins, _ 1 _)
    let mut occ: HashMap<(u64, u64, u64, u64), (u64, u64)> = HashMap::new();
    calculate_occ(pawn0, pawn1, 0, 0, &mut occ);

    let (i0, i1) = occ.get(&(pawn0, pawn1, 0, 0)).unwrap();

    // println!("{:?}", (i0, i1));

    Solution::from(if i0 > i1 { *i0 as i64 } else { *i1 as i64 })
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    // retrieves int from 0 to 10 from end of a string
    let foo = |l: &String| -> u64 {
        match l.chars().last().unwrap() {
            '0' => 10,
            el => el.to_digit(10).unwrap() as u64,
        }
    };

    (foo(&input[0]), foo(&input[1]))
}
