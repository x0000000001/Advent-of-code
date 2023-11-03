use crate::Solution;

fn simulate(initial_fishes: &Vec<i64>, days: i64) -> i64 {
    //counts fishes at stage x, x from 0 to 8
    let mut fishes: Vec<i64> = vec![0; 9];

    for f in initial_fishes {
        fishes[*f as usize] += 1;
    }

    for _ in 0..days {
        let temp = fishes[0];

        for i in 0..8 {
            fishes[i] = fishes[i + 1];
        }

        fishes[6] += temp;
        fishes[8] = temp;
    }

    fishes.iter().sum()
}

pub fn part1(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let initial_fishes: Vec<i64> = input[0].split(',').map(|el| el.parse().unwrap()).collect();

    Solution::from(simulate(&initial_fishes, 80))
}

pub fn part2(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let initial_fishes: Vec<i64> = input[0].split(',').map(|el| el.parse().unwrap()).collect();

    Solution::from(simulate(&initial_fishes, 256))
}
