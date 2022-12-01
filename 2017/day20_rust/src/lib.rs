use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Add,
};

pub type InputType = Vec<Particle>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn magnitude(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    p: Point,
    v: Point,
    a: Point,
}

impl Particle {
    // measures the point that'll stay closer to the origin on the long term
    // -1 if this one, 0 if equal, 1 if other
    fn compare(&self, other: &Particle) -> i64 {
        let a_comp = i64::signum(self.a.magnitude() - other.a.magnitude());

        if a_comp != 0 {
            return a_comp;
        }

        let v_comp = i64::signum(self.v.magnitude() - other.v.magnitude());

        if v_comp != 0 {
            return v_comp;
        }

        let p_comp = i64::signum(self.p.magnitude() - other.p.magnitude());

        if p_comp != 0 {
            return p_comp;
        }

        0
    }
}

pub fn result_1(input: InputType) -> i64 {
    let mut els = input
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, Particle)>>();
    els.sort_by(|(_, p0), (_, p1)| match p0.compare(p1) {
        -1 => std::cmp::Ordering::Less,
        0 => std::cmp::Ordering::Equal,
        1 => std::cmp::Ordering::Greater,
        _ => panic!(),
    });

    els[0].0 as i64
}

#[derive(PartialEq, Eq, Debug)]
enum Intersection {
    Always,
    None,
    Some(i64),
}

// Another attempt to be smart
// Once again, not a success

// fn compute_1d_intersection(p0: i64, v0: i64, a0: i64, p1: i64, v1: i64, a1: i64) -> Intersection {
//     if a0 == a1 {
//         // 1st degree equation
//         // 0 degree equation
//         if v1 == v0 {
//             if p0 == p1 {
//                 Intersection::Always
//             } else {
//                 Intersection::None
//             }
//         } else {
//             // println!("2nd degree");
//             // println!("{} {}",(p1-p0).abs(),(v0-v1).abs());
//             if (p1 - p0).abs() % (v0 - v1).abs() == 0 {
//                 let t = (p1 - p0) / (v0 - v1);
//                 if t >= 0 {
//                     Intersection::Some(t)
//                 } else {
//                     Intersection::None
//                 }
//             } else {
//                 Intersection::None
//             }
//         }
//     } else {
//         // 2nd degree equation

//         let a = (a1 - a0) as f64;
//         let b = (2 * (v1 - v0) - (a1 - a0)) as f64;
//         let c = (2 * (p1 - p0)) as f64;

//         let delta = b * b - 4_f64 * a * c;

//         // println!("{delta}");

//         if delta < 0.0 {
//             Intersection::None
//         } else {
//             // println!("yes");
//             let sqrt_delta = delta.sqrt();

//             let tminus = (-b - sqrt_delta) / (2_f64 * a);
//             let t_minus_int = tminus.floor() as i64;

//             // println!("{tminus}");
//             // println!("{t_minus_int}");

//             if t_minus_int > 0
//                 && (t_minus_int * (t_minus_int - 1)) / 2 * (a1 - a0)
//                     + t_minus_int * (v1 - v0)
//                     + (p1 - p0)
//                     == 0
//             {
//                 return Intersection::Some(t_minus_int);
//             }

//             let tplus = (-b + sqrt_delta) / (2_f64 * a);
//             let t_plus_int = tplus.floor() as i64;

//             // println!("{tplus}");
//             // println!("{t_plus_int}");

//             if t_plus_int > 0
//                 && (t_plus_int * (t_plus_int - 1)) / 2 * (a1 - a0)
//                     + t_plus_int * (v1 - v0)
//                     + (p1 - p0)
//                     == 0
//             {
//                 return Intersection::Some(t_plus_int);
//             }

//             Intersection::None
//         }
//     }
// }

// fn compute_particle_intersect(p0: &Particle, p1: &Particle) -> Intersection {
//     let x_inter = compute_1d_intersection(p0.p.x, p0.v.x, p0.a.x, p1.p.x, p1.v.x, p1.a.x);
//     let y_inter = compute_1d_intersection(p0.p.y, p0.v.y, p0.a.y, p1.p.y, p1.v.y, p1.a.y);
//     let z_inter = compute_1d_intersection(p0.p.z, p0.v.z, p0.a.z, p1.p.z, p1.v.z, p1.a.z);

//     if x_inter == Intersection::None
//         || y_inter == Intersection::None
//         || z_inter == Intersection::None
//     {
//         return Intersection::None;
//     }

//     if x_inter == Intersection::Always
//         && y_inter == Intersection::Always
//         && z_inter == Intersection::Always
//     {
//         return Intersection::Always;
//     }

//     let mut possible_times: Vec<i64> = [x_inter, y_inter, z_inter]
//         .into_iter()
//         .filter(|inter| match inter {
//             Intersection::Some(t) => *t >= 0,
//             _ => false,
//         })
//         .map(|inter| match inter {
//             Intersection::Some(t) => t,
//             _ => panic!(),
//         })
//         .collect();

//     possible_times.dedup();

//     if possible_times.len() == 1 {
//         Intersection::Some(possible_times[0])
//     } else {
//         Intersection::None
//     }
// }

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// not smart, but good enough
fn compute_particle_intersect(mut p0: Particle, mut p1: Particle) -> Intersection {
    for t in 0..100 {
        if p0.p == p1.p {
            return Intersection::Some(t);
        }

        p0.v = p0.v + p0.a;
        p0.p = p0.p + p0.v;
        p1.v = p1.v + p1.a;
        p1.p = p1.p + p1.v;
    }

    Intersection::None
}

pub fn result_2(input: InputType) -> i64 {
    //(collisson time, (p0_index,p1_index))
    let mut collisions: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..input.len() {
        for j in 0..i {
            // println!("{} {} {:?}", i, j, compute_particle_intersect(&input[i], &input[j]));
            let intersect = compute_particle_intersect(input[i], input[j]);
            match intersect {
                Intersection::Some(t) => {
                    let entry = collisions.entry(t as u64).or_insert(Vec::new());
                    entry.push((i, j));
                }
                Intersection::Always => {
                    let entry = collisions.entry(0).or_insert(Vec::new());
                    entry.push((i, j));
                }
                Intersection::None => (),
            }
        }
    }

    let mut remaining_set: HashSet<usize> = HashSet::new();

    for i in 0..input.len() {
        remaining_set.insert(i);
    }

    let mut collisions: Vec<(u64, Vec<(usize, usize)>)> = collisions.into_iter().collect();
    collisions.sort_by(|(t0, _), (t1, _)| t0.cmp(t1));
    print!("");
    for (_, collision_pairs) in collisions {
        let mut to_remove = Vec::new();
        for (i, j) in collision_pairs {
            if !remaining_set.contains(&i) || !remaining_set.contains(&j) {
                continue;
            }
            to_remove.push(i);
            to_remove.push(j);
        }

        for v in to_remove.into_iter() {
            remaining_set.remove(&v);
        }
    }

    remaining_set.len() as i64
}

pub fn read_input(path: &str) -> InputType {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    fn extract_num(s: &str) -> Point {
        let w = s.split("<").collect::<Vec<&str>>();
        let w1 = w[1]
            .replace(">", "")
            .split(",")
            .map(|el| el.parse().unwrap())
            .collect::<Vec<i64>>();
        Point {
            x: w1[0],
            y: w1[1],
            z: w1[2],
        }
    }

    for l in input {
        let points = l
            .split_whitespace()
            .map(|w| &w[0..w.len() - 1])
            .collect::<Vec<&str>>();
        res.push(Particle {
            p: extract_num(points[0]),
            v: extract_num(points[1]),
            a: extract_num(points[2]),
        })
    }

    res
}
