use std::{collections::VecDeque, fs, io::stdin};

pub type InputType = Vec<NanoBot>;

#[derive(Debug, Clone, Copy)]
pub struct Point(i64, i64, i64);

#[derive(Debug)]
pub struct NanoBot {
    pos: Point,
    radius: i64,
}

fn manhattan(Point(x0, y0, z0): &Point, Point(x1, y1, z1): &Point) -> i64 {
    (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()
}

fn is_in_range(ref_nanobot: &NanoBot, nanobot: &NanoBot) -> bool {
    manhattan(&ref_nanobot.pos, &nanobot.pos) <= ref_nanobot.radius
}

pub fn result_1(input: InputType) -> i64 {
    let powerful_nanobot = input.iter().max_by_key(|n| n.radius).unwrap();

    input
        .iter()
        .filter(|n| is_in_range(powerful_nanobot, n))
        .count() as i64
}

impl NanoBot {
    pub fn center(nanobots: &Vec<&NanoBot>) -> Point {
        let (sumx, sumy, sumz) = nanobots.iter().fold((0, 0, 0), |(acc0, acc1, acc2), n| {
            (acc0 + n.pos.0, acc1 + n.pos.1, acc2 + n.pos.2)
        });

        let l = nanobots.len() as i64;

        Point(sumx / l, sumy / l, sumz / l)
    }

    pub fn is_center_contained(nanobots: &Vec<&NanoBot>) -> bool {
        let center = NanoBot::center(nanobots);
        let mut b = true;

        for n in nanobots.iter() {
            if manhattan(&center, &n.pos) > n.radius {
                b = false;
                break;
            }
        }

        b
    }
}

pub fn max_combination<'a>(nanobots: &Vec<&'a NanoBot>) -> Vec<&'a NanoBot> {
    if nanobots.len() % 100 == 0 {
        println!("{:?}", nanobots.len());
    }
    if nanobots.len() == 1 {
        return nanobots.clone();
    }

    if NanoBot::is_center_contained(nanobots) {
        return nanobots.clone();
    }

    let mut others = nanobots.clone();
    let last_nanobot = others.pop().unwrap();
    let best_combination_without_last = max_combination(&others);

    let mut touching_nanobots: Vec<&NanoBot> = others
        .into_iter()
        .filter(|n| manhattan(&n.pos, &last_nanobot.pos) <= n.radius + last_nanobot.radius)
        .collect();

    touching_nanobots.push(last_nanobot);

    if touching_nanobots.len() == nanobots.len() {
        return nanobots.clone();
    }

    let best_combination_with_last = max_combination(&touching_nanobots);

    if best_combination_with_last.len() > best_combination_without_last.len() {
        best_combination_with_last
    } else {
        best_combination_without_last
    }
}

// 416618662 too high
pub fn result_2(nanobots: InputType) -> i64 {
    let c = max_combination(&nanobots.iter().collect());
    println!("{:?}", c);

    c.len() as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let parse_line = |line: &str| -> NanoBot {
        let parts = line.split(['<', ',', '>', '=']).collect::<Vec<&str>>();
        NanoBot {
            pos: Point(
                parts[2].parse().unwrap(),
                parts[3].parse().unwrap(),
                parts[4].parse().unwrap(),
            ),
            radius: parts[7].parse().unwrap(),
        }
    };

    contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .filter(|l| !l.is_empty())
        .map(|l| parse_line(&l))
        .collect()
}

// FIRST TRY

/*

let origin = Point(0, 0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(find_biggest_zone(&nanobots));
    let mut current_min = i64::MAX;
    let mut max_score = 0;
    let mut i = 0;
    let mut zonee = Zone {
        bottom_left: Point(0, 0, 0),
        x_length: 0,
        y_length: 0,
        z_length: 0,
    };

    let mut t = String::new();

    while let Some(zone) = queue.pop_front() {
        i += 1;
        let score = nanobots_containing_zone(&nanobots, &zone);
        let potential_score = nanobots_touching_zone(&nanobots, &zone);
        // println!(
        //     "zone : {:?}\nscore : {}\npotential_score : {}\nmax_score : {}\n",
        //     zone, score, potential_score, max_score
        // );

        if potential_score < max_score {
            continue;
        }

        if zone.is_a_point() || potential_score == score {
            let distance = manhattan(&origin, &zone.bottom_left);
            // find nearest point covering everything
            if (score > max_score) || (score == max_score && distance <= current_min) {
                current_min = distance;
                zonee = zone;
                max_score = score;
            }
        } else {
            max_score = score.max(max_score);
            zone.split()
                .into_iter()
                .for_each(|child| queue.push_back(child));
        }
        // stdin().read_line(&mut t).expect("no");
    }
    println!("{}", i);
    println!("{}", max_score);
    println!("{:?}", zonee);
    current_min

#[derive(Debug, Clone, Copy)]
pub struct Zone {
    bottom_left: Point,
    x_length: i64,
    y_length: i64,
    z_length: i64,
}

impl NanoBot {
    pub fn touches_zone(&self, zone: &Zone) -> bool {
        // a nanobot touches a rectangular zone only and only if it covers
        // one of its vertices, hence the 8 corners check, or is inside the zone

        if zone.contains_point(&self.pos) {
            return true;
        }

        for x_add in 0..2 {
            for y_add in 0..2 {
                for z_add in 0..2 {
                    let x = zone.bottom_left.0 + x_add * zone.x_length;
                    let y = zone.bottom_left.1 + y_add * zone.y_length;
                    let z = zone.bottom_left.2 + z_add * zone.z_length;
                    let corner = Point(x, y, z);

                    if manhattan(&corner, &self.pos) <= self.radius {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn contains_zone(&self, zone: &Zone) -> bool {
        // a nanobot touches a rectangular zone only and only if it covers
        // one of its vertices, hence the 8 corners check

        for x_add in 0..2 {
            for y_add in 0..2 {
                for z_add in 0..2 {
                    let x = zone.bottom_left.0 + x_add * zone.x_length;
                    let y = zone.bottom_left.1 + y_add * zone.y_length;
                    let z = zone.bottom_left.2 + z_add * zone.z_length;
                    let corner = Point(x, y, z);

                    if manhattan(&corner, &self.pos) > self.radius {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl Zone {
    pub fn split(&self) -> Vec<Zone> {
        let (x_length_0, x_length_1) = if self.x_length == 1 {
            (0, 0)
        } else {
            (self.x_length / 2, self.x_length - (self.x_length / 2))
        };

        let (y_length_0, y_length_1) = if self.y_length == 1 {
            (0, 0)
        } else {
            (self.y_length / 2, self.y_length - (self.y_length / 2))
        };
        let (z_length_0, z_length_1) = if self.z_length == 1 {
            (0, 0)
        } else {
            (self.z_length / 2, self.z_length - (self.z_length / 2))
        };

        let mut children = vec![];

        for x_add in 0..2 {
            let mut x = self.bottom_left.0 + x_add * x_length_0;
            let x_length = if x_add == 0 { x_length_0 } else { x_length_1 };
            if self.x_length == 1 && x_add == 1 {
                x += 1;
            }

            for y_add in 0..2 {
                let mut y = self.bottom_left.1 + y_add * y_length_0;
                let y_length = if y_add == 0 { y_length_0 } else { y_length_1 };
                if self.y_length == 1 && y_add == 1 {
                    y += 1;
                }

                for z_add in 0..2 {
                    let mut z = self.bottom_left.2 + z_add * z_length_0;
                    let z_length = if z_add == 0 { z_length_0 } else { z_length_1 };
                    if self.z_length == 1 && z_add == 1 {
                        z += 1;
                    }

                    children.push(Zone {
                        bottom_left: Point(x, y, z),
                        x_length,
                        y_length,
                        z_length,
                    })
                }
            }
        }

        children
    }

    pub fn is_a_point(&self) -> bool {
        self.x_length == 0 && self.y_length == 0 && self.z_length == 0
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        (point.0 >= self.bottom_left.0 && point.0 <= self.bottom_left.0 + self.x_length)
            && (point.1 >= self.bottom_left.1 && point.1 <= self.bottom_left.1 + self.y_length)
            && (point.0 >= self.bottom_left.2 && point.2 <= self.bottom_left.2 + self.z_length)
    }
}

fn find_biggest_zone(nanobots: &InputType) -> Zone {
    // looking only in the nanobots without the ranges,
    // because the point we are looking for has to be inside a square of beacons
    // not efficient but it's only init
    let min_x = nanobots.iter().max_by_key(|n| -n.pos.0).unwrap().pos.0;
    let max_x = nanobots.iter().max_by_key(|n| n.pos.0).unwrap().pos.0;
    let min_y = nanobots.iter().max_by_key(|n| -n.pos.1).unwrap().pos.1;
    let max_y = nanobots.iter().max_by_key(|n| n.pos.1).unwrap().pos.1;
    let min_z = nanobots.iter().max_by_key(|n| -n.pos.2).unwrap().pos.2;
    let max_z = nanobots.iter().max_by_key(|n| n.pos.2).unwrap().pos.2;

    Zone {
        bottom_left: Point(min_x, min_y, min_z),
        x_length: max_x - min_x,
        y_length: max_y - min_y,
        z_length: max_z - min_z,
    }
}

fn nanobots_touching_zone(nanobots: &InputType, zone: &Zone) -> usize {
    nanobots.iter().filter(|n| n.touches_zone(zone)).count()
}

fn nanobots_containing_zone(nanobots: &InputType, zone: &Zone) -> usize {
    nanobots.iter().filter(|n| n.contains_zone(zone)).count()
}


*/
