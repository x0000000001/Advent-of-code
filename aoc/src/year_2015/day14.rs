use std::collections::HashMap;

use crate::Solution;

pub struct Reindeer {
    speed: i64,
    fly_time: i64,
    rest_time: i64,
    distance_travelled: i64,
    fly_time_remaining: i64,
    rest_time_remaining: i64,
}

type InputType = HashMap<String, Reindeer>;

impl Reindeer {
    pub fn create(speed: i64, fly_time: i64, rest_time: i64) -> Reindeer {
        Reindeer {
            speed,
            fly_time,
            rest_time,
            distance_travelled: 0,
            fly_time_remaining: fly_time,
            rest_time_remaining: 0,
        }
    }

    pub fn step(&mut self) {
        if self.fly_time_remaining > 0 {
            self.fly_time_remaining -= 1;
            self.distance_travelled += self.speed;
            // needs to rest
            if self.fly_time_remaining == 0 {
                self.rest_time_remaining = self.rest_time;
            }
        } else {
            self.rest_time_remaining -= 1;
            // can fly again
            if self.rest_time_remaining == 0 {
                self.fly_time_remaining = self.fly_time;
            }
        }
    }

    pub fn get_score(&self) -> i64 {
        self.distance_travelled
    }
}

pub fn part1(s: String) -> Solution {
    let mut input = parse(s);

    for _ in 0..2503 {
        for (_, r) in input.iter_mut() {
            r.step();
        }
    }

    Solution::from(
        input
            .iter()
            .map(|(_, r)| r.distance_travelled)
            .max()
            .unwrap(),
    )
}

pub fn part2(s: String) -> Solution {
    let mut input = parse(s);
    let mut scores: HashMap<String, i64> = HashMap::new();

    for _ in 0..2503 {
        for (_, r) in input.iter_mut() {
            r.step();
        }

        let max = input
            .iter()
            .map(|(_, r)| r.distance_travelled)
            .max()
            .unwrap();

        for (n, r) in input.iter() {
            if r.distance_travelled == max {
                let accessor = scores.entry(n.clone()).or_insert(0);
                *accessor += 1;
            }
        }
    }

    Solution::from(scores.into_iter().map(|(_, s)| s).max().unwrap())
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = HashMap::new();

    for l in input {
        let words: Vec<&str> = l.split_whitespace().collect();
        let speed: i64 = words[3].parse().unwrap();
        let fly_time: i64 = words[6].parse().unwrap();
        let rest_time: i64 = words[13].parse().unwrap();

        res.insert(
            words[0].to_string(),
            Reindeer::create(speed, fly_time, rest_time),
        );
    }

    res
}
