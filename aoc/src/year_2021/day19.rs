use std::collections::HashMap;

use crate::Solution;

type InputType = Vec<Scanner>;

#[derive(Debug)]
struct Scanner {
    // -1 : -x, -2 : -y, -3 : -z, 1 : +x, 2 : +y, 3 +z
    facing: i64,
    // between 1 and 4
    orientation: i64,
    offset: (i64, i64, i64),
    beams: Vec<(i64, i64, i64)>,
}

impl Scanner {
    // returns beam coordinates according to orientation and offset
    fn read_beam(&self, i: usize) -> (i64, i64, i64) {
        let (mut x, mut y, mut z) = self.beams[i];

        match self.orientation {
            1 => (),
            2 => (y, z) = (z, -y),
            3 => (y, z) = (-y, -z),
            4 => (y, z) = (-z, y),
            _ => (),
        }

        match self.facing {
            1 => (x, y, z) = (x, y, z),
            2 => (x, y, z) = (y, -x, z),
            -1 => (x, y, z) = (-x, -y, z),
            -2 => (x, y, z) = (-y, x, z),
            3 => (x, y, z) = (z, y, -x),
            -3 => (x, y, z) = (-z, y, x),
            _ => (),
        }

        x += self.offset.0;
        y += self.offset.1;
        z += self.offset.2;

        (x, y, z)
    }
}

// counts common beams between two scanners
fn common_beams(s0: &Scanner, s1: &Scanner) -> i64 {
    let mut count = 0;

    for i in 0..s0.beams.len() {
        for j in 0..s1.beams.len() {
            if s0.read_beam(i) == s1.read_beam(j) {
                // println!("{:?}",s0.read_beam(i));
                count += 1;
            }
        }
    }

    count
}

// trying to align s0 to s1, returning bool stating if alignement was successfull
fn try_align(s0: &mut Scanner, s1: &mut Scanner) -> bool {
    // println!("{:?}", s0);
    // println!("{:?}", s1);

    for facing in [-3, -2, -1, 1, 2, 3] {
        s0.facing = facing;
        for orientation in [1, 2, 3, 4] {
            s0.orientation = orientation;

            // counting number of times each offset appears between each points of both scanners
            let mut offsets: HashMap<(i64, i64, i64), i64> = HashMap::new();

            for (x0, y0, z0) in (0..s0.beams.len()).map(|i| s0.read_beam(i)) {
                for (x1, y1, z1) in (0..s1.beams.len()).map(|i| s1.read_beam(i)) {
                    let accessor = offsets.entry((x1 - x0, y1 - y0, z1 - z0)).or_insert(0);
                    *accessor += 1;
                }
            }

            // println!("{:?}", offsets);

            for (offset, count) in offsets.into_iter() {
                if count >= 12 {
                    s0.offset = offset;

                    if common_beams(s0, s1) >= 12 {
                        // println!("offset : {:?}, facing : {facing}, orientation : {orientation}", offset);
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn align_scanners(scanners: &mut Vec<Scanner>) {
    let mut aligned_scanners: Vec<Scanner> = vec![];
    let mut last_scanners_aligned: Vec<Scanner> = vec![];

    last_scanners_aligned.push(scanners.swap_remove(0));

    // println!("scanners : {:?}", scanners);
    // println!("aligned scanners : {:?}", aligned_scanners);
    // println!("last aligned scanners : {:?}", last_scanners_aligned);

    // TODO : transfer elements from scanners to align scanners with try_align criteria
    while !scanners.is_empty() {
        let mut i: i64 = 0;
        let mut new_scanners_aligned: Vec<Scanner> = vec![];

        while (i as usize) < scanners.len() {
            for j in 0..last_scanners_aligned.len() {
                if try_align(&mut scanners[i as usize], &mut last_scanners_aligned[j]) {
                    new_scanners_aligned.push(scanners.remove(i as usize));
                    i -= 1;
                    break;
                }
            }

            i += 1;
        }

        aligned_scanners.append(&mut last_scanners_aligned);
        last_scanners_aligned.append(&mut new_scanners_aligned);
    }

    aligned_scanners.append(&mut last_scanners_aligned);

    scanners.append(&mut aligned_scanners);
}

pub fn part1(s: String) -> Solution {
    let mut scanners = parse(s);

    align_scanners(&mut scanners);

    let mut positions: Vec<(i64, i64, i64)> = vec![];
    for s in scanners {
        for pos in (0..s.beams.len()).map(|i| s.read_beam(i)) {
            positions.push(pos);
        }
    }

    positions.sort();
    positions.dedup();

    Solution::from(positions.len() as i64)
}

fn manhattan_distance((x0, y0, z0): (i64, i64, i64), (x1, y1, z1): (i64, i64, i64)) -> i64 {
    (x1 - x0).abs() + (y1 - y0).abs() + (z1 - z0).abs()
}

pub fn part2(s: String) -> Solution {
    let mut scanners = parse(s);

    align_scanners(&mut scanners);

    let mut max = 0;

    for i in 0..scanners.len() {
        for j in (i + 1)..scanners.len() {
            let d = manhattan_distance(scanners[i].offset, scanners[j].offset);

            if d > max {
                max = d;
            }
        }
    }

    Solution::from(max)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut scanners: Vec<Scanner> = vec![];
    let mut i = 1;

    while i < input.len() {
        let mut s = Scanner {
            facing: 0,
            orientation: 0,
            offset: (0, 0, 0),
            beams: vec![],
        };

        while i < input.len() && !input[i].is_empty() {
            let v: Vec<i64> = input[i]
                .split(',')
                .map(|el| el.parse::<i64>().unwrap())
                .collect();
            s.beams.push((v[0], v[1], v[2]));

            i += 1;
        }

        scanners.push(s);

        i += 2;
    }

    scanners
}
