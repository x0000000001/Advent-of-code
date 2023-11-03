use crate::Solution;

pub fn part1(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let positions: Vec<i32> = input[0].split(',').map(|el| el.parse().unwrap()).collect();

    let mut min = std::i32::MAX;

    let max_pos = positions.iter().max().unwrap();

    for i in 0..*max_pos {
        let cost: i32 = positions.iter().map(|el| (el - i).abs()).sum();

        if cost < min {
            min = cost;
        }
    }

    Solution::from(min)
}

pub fn part2(s: String) -> Solution {
    let input: Vec<_> = s.lines().map(|line| line.trim()).collect();

    let positions: Vec<i32> = input[0].split(',').map(|el| el.parse().unwrap()).collect();

    let mut min = std::i32::MAX;

    let max_pos = positions.iter().max().unwrap();

    for i in 0..*max_pos {
        let cost: i32 = positions
            .iter()
            .map(|el| ((el - i).abs() * ((el - i).abs() + 1)) / 2)
            .sum();

        if cost < min {
            min = cost;
        }
    }

    Solution::from(min)
}
