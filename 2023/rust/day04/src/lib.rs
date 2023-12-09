mod helpers;

pub use helpers::Solution;

struct Card {
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

type InputType = Vec<Card>;

pub fn part1(s: String) -> Solution {
    let cards = parse(s);

    Solution::from(
        cards
            .into_iter()
            .map(|c| c.numbers.iter().filter(|n| c.winning.contains(n)).count())
            .map(|x| if x == 0 { 0 } else { 2_u64.pow((x - 1) as u32) })
            .sum::<u64>(),
    )
}

pub fn part2(s: String) -> Solution {
    let cards = parse(s);

    let winning_counts = cards
        .iter()
        .map(|c| c.numbers.iter().filter(|n| c.winning.contains(n)).count())
        .collect::<Vec<usize>>();

    let mut cards_counts = vec![1; cards.len()];

    for i in 0..cards.len() {
        for j in (i + 1)..((i + winning_counts[i] + 1).min(cards.len())) {
            cards_counts[j] += cards_counts[i];
        }
    }

    Solution::from(cards_counts.iter().sum::<usize>() as u64)
}

fn parse(s: String) -> InputType {
    let numbers_from_s =
        |s: &str| -> Vec<usize> { s.split_whitespace().map(|w| w.parse().unwrap()).collect() };

    s.lines()
        .map(|l| {
            let w0 = l.split(':').collect::<Vec<&str>>();
            let w01 = w0[1].split('|').collect::<Vec<&str>>();

            Card {
                winning: numbers_from_s(w01[0]),
                numbers: numbers_from_s(w01[1]),
            }
        })
        .collect()
}
