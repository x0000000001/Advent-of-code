use std::collections::HashMap;

// name, speed, fly time, rest time
pub type InputType = HashMap<String, Reindeer>;

pub struct Reindeer {
    speed: i64,
    fly_time: i64,
    rest_time: i64,
    distance_travelled: i64,
    fly_time_remaining: i64,
    rest_time_remaining: i64,
}

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

pub fn result_1(mut input: InputType) -> i64 {
    for _ in 0..2503 {
        for (_, r) in input.iter_mut() {
            r.step();
        }
    }

    input
        .iter()
        .map(|(_, r)| r.distance_travelled)
        .max()
        .unwrap()
}

pub fn result_2(mut input: InputType) -> i64 {
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

    scores.into_iter().map(|(n, s)| s).max().unwrap()
}
