use std::collections::BinaryHeap;
use std::fs;
use std::ops;

pub type InputType = Vec<NanoBot>;

#[derive(Debug, Clone, Copy)]
pub struct Point(i64, i64, i64);

#[derive(Debug)]
pub struct NanoBot {
    pos: Point,
    radius: usize,
}

impl ops::Add<i64> for Point {
    type Output = Point;

    fn add(self, rhs: i64) -> Self::Output {
        Point(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, p: Point) -> Self::Output {
        Point(self.0 + p.0, self.1 + p.1, self.2 + p.2)
    }
}

impl Point {
    fn is_in_cube(&self, cube: &Cube) -> bool {
        self.0 >= cube.bbb.0
            && self.0 <= cube.tbb.0
            && self.1 >= cube.bbb.1
            && self.1 <= cube.btb.1
            && self.2 >= cube.bbb.2
            && self.2 <= cube.bbt.2
    }
}

fn manhattan(Point(x0, y0, z0): &Point, Point(x1, y1, z1): &Point) -> usize {
    ((x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()) as usize
}

fn is_in_range(ref_nanobot: &NanoBot, nanobot: &NanoBot) -> bool {
    manhattan(&ref_nanobot.pos, &nanobot.pos) <= ref_nanobot.radius
}

fn powerful_nanobot_score(input: &InputType) -> usize {
    let powerful_nanobot = input.iter().max_by_key(|n| n.radius).unwrap();

    input
        .iter()
        .filter(|n| is_in_range(powerful_nanobot, n))
        .count()
}

pub fn result_1(nanobots: InputType) -> i64 {
    powerful_nanobot_score(&nanobots) as i64
}

#[derive(Debug)]
struct Cube {
    //xyz
    bbb: Point,
    bbt: Point,
    btb: Point,
    btt: Point,
    tbb: Point,
    tbt: Point,
    ttb: Point,
    ttt: Point,
    x_width: usize,
    y_width: usize,
    z_width: usize,
    max_possible_radius: usize,
}

impl Cube {
    fn from(bottom: Point, xwidth: usize, ywidth: usize, zwidth: usize) -> Cube {
        Cube {
            bbb: bottom,
            bbt: Point(bottom.0, bottom.1, bottom.2 + zwidth as i64),
            btb: Point(bottom.0, bottom.1 + ywidth as i64, bottom.2),
            btt: Point(bottom.0, bottom.1 + ywidth as i64, bottom.2 + zwidth as i64),
            tbb: Point(bottom.0 + xwidth as i64, bottom.1, bottom.2),
            tbt: Point(bottom.0 + xwidth as i64, bottom.1, bottom.2 + zwidth as i64),
            ttb: Point(bottom.0 + xwidth as i64, bottom.1 + ywidth as i64, bottom.2),
            ttt: Point(
                bottom.0 + xwidth as i64,
                bottom.1 + ywidth as i64,
                bottom.2 + xwidth as i64,
            ),
            x_width: xwidth,
            y_width: ywidth,
            z_width: zwidth,
            max_possible_radius: xwidth + ywidth + zwidth,
        }
    }

    fn edges<'a>(&'a self) -> impl Iterator<Item = &'a Point> {
        [
            &self.bbb, &self.bbt, &self.btb, &self.btt, &self.tbb, &self.tbt, &self.ttb, &self.ttt,
        ]
        .into_iter()
    }

    fn split(&self) -> Vec<Cube> {
        assert!(!self.is_point());

        let xwidths = [self.x_width / 2, self.x_width - (self.x_width / 2)];
        let ywidths = [self.y_width / 2, self.y_width - (self.y_width / 2)];
        let zwidths = [self.z_width / 2, self.z_width - (self.z_width / 2)];
        let mut children = vec![];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    children.push(Cube::from(
                        Point(
                            self.bbb.0 + i as i64 * xwidths[i] as i64,
                            self.bbb.1 + j as i64 * ywidths[j] as i64,
                            self.bbb.2 + k as i64 * zwidths[k] as i64,
                        ),
                        xwidths[i],
                        ywidths[j],
                        zwidths[k],
                    ));
                }
            }
        }

        children
    }

    fn is_point(&self) -> bool {
        self.x_width == 0 && self.y_width == 0 && self.z_width == 0
    }
}

impl NanoBot {
    fn touches_point(&self, point: &Point) -> bool {
        manhattan(&self.pos, point) <= self.radius
    }

    // touches, contains
    fn touches_or_contains_cube(&self, cube: &Cube) -> (bool, bool) {
        let contacts = cube.edges().map(|e| self.touches_point(e));

        let mut can_touch_cube = self.pos.is_in_cube(cube);

        if !can_touch_cube {
            let max_distance = cube.edges().map(|e| manhattan(e, &self.pos)).max().unwrap();
            if max_distance <= self.radius + cube.max_possible_radius {
                can_touch_cube = true;
            }
        }

        (can_touch_cube, contacts.fold(true, |acc, b| acc && b))
    }
}

#[derive(Debug)]
struct CubeScore {
    cube: Cube,
    parent_nanobots_touched: usize,
}

impl PartialEq for CubeScore {
    fn eq(&self, other: &Self) -> bool {
        self.parent_nanobots_touched == other.parent_nanobots_touched
    }
}

impl Eq for CubeScore {}

impl PartialOrd for CubeScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.parent_nanobots_touched
            .partial_cmp(&other.parent_nanobots_touched)
    }
}

impl Ord for CubeScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.parent_nanobots_touched
            .cmp(&other.parent_nanobots_touched)
    }
}

fn optimal_position(nanobots: &InputType) -> usize {
    let mut minimum = 0; //873;
    let (xmin, xmax, ymin, ymax, zmin, zmax) = nanobots.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |(xmin, xmax, ymin, ymax, zmin, zmax), n| {
            (
                xmin.min(n.pos.0),
                xmax.max(n.pos.0),
                ymin.min(n.pos.1),
                ymax.max(n.pos.1),
                zmin.min(n.pos.2),
                zmax.max(n.pos.2),
            )
        },
    );

    let mut optimal_distance = usize::MAX;
    let mut queue: BinaryHeap<CubeScore> = BinaryHeap::new();

    queue.push(CubeScore {
        cube: Cube::from(
            Point(xmin, ymin, zmin),
            (xmax - xmin) as usize,
            (ymax - ymin) as usize,
            (zmax - zmin) as usize,
        ),
        parent_nanobots_touched: 0,
    });

    while let Some(CubeScore {
        cube: candidate,
        parent_nanobots_touched: _,
    }) = queue.pop()
    {
        let (nanobots_touched, nanobots_contained) = nanobots
            .iter()
            .map(|n| n.touches_or_contains_cube(&candidate))
            .fold((0, 0), |(touched, contained), (t, c)| {
                (touched + t as usize, contained + c as usize)
            });

        if nanobots_touched < minimum {
            continue;
        }

        if nanobots_contained > minimum {
            minimum = nanobots_contained;
            optimal_distance = usize::MAX;
        }

        if candidate.is_point() {
            if nanobots_contained == minimum {
                optimal_distance = manhattan(&candidate.bbb, &Point(0, 0, 0)).min(optimal_distance);
            }
        } else {
            for c in candidate.split() {
                queue.push(CubeScore {
                    cube: c,
                    parent_nanobots_touched: nanobots_touched,
                });
            }
        }
    }

    optimal_distance
}

// 416618662 too high
pub fn result_2(nanobots: InputType) -> i64 {
    optimal_position(&nanobots) as i64
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
