use std::fs;

const FILE: &str = "input.txt";

#[derive(Debug, Clone, Copy)]
enum Order {
    On,
    Off,
    Toggle
}

#[derive(Debug, Clone, Copy)]
struct Instr {
    order: Order,
    start: (i64,i64),
    end: (i64,i64)
}

impl Instr {
    fn area(&self) -> i64
    {
        (self.end.1 - self.start.1 + 1) * (self.end.0 - self.start.0 + 1)
    }

    fn intersection(&self, other: &Instr) -> Option<Instr>
    {
        if self.start.0 > other.end.0 || self.end.0 < other.start.0 || self.start.1 > other.end.1 || self.end.1 < other.start.1{
            None
        } 

        else {
            Some(
            Instr {
                order: match other.order {
                    Order::Toggle => Order::Toggle,
                    Order::On => Order::On,
                    Order::Off => Order::Off
                },
                start: (self.start.0.max(other.start.0), self.start.1.max(other.start.1)),
                end: (self.end.0.min(other.end.0), self.end.1.min(other.end.1))
            })
        }
    }
}

fn read_input() -> Vec<Instr>
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let mut  instrs: Vec<Instr> = vec![];

    let get_coord = |s: &str| -> (i64,i64) {
        let temp = s.split(',').map(|el| el.parse().unwrap()).collect::<Vec<i64>>();
        (temp[0],temp[1])
    }; 

    for l in input {
        let words = l.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "toggle" => {
                let start = get_coord(words[1]);
                let end = get_coord(words[3]);

                instrs.push(Instr{
                    order: Order::Toggle,
                    start,
                    end
                });
            },
            "turn" => {
                let start = get_coord(words[2]);
                let end = get_coord(words[4]);
                instrs.push(Instr{
                    order: match words[1] { "on" => Order::On, _ => Order::Off},
                    start,
                    end
                });
            }, 
            _ => ()
        }
    }

    instrs
}

pub fn result_1() -> i64
{
    let instrs = read_input();
    let mut lights: Vec<Vec<bool>> = vec![vec![false;1000];1000];

    for l in instrs {
        match l.order {
            Order::On => {
                for i in l.start.0..(l.end.0+1) {
                    for j in l.start.1..(l.end.1+1){
                        lights[i as usize][j as usize] = true;
                    }
                }
            },
            Order::Off => {
                for i in l.start.0..(l.end.0+1) {
                    for j in l.start.1..(l.end.1+1){
                        lights[i as usize][j as usize] = false;
                    }
                }
            },
            Order::Toggle => {
                for i in l.start.0..(l.end.0+1) {
                    for j in l.start.1..(l.end.1+1){
                        lights[i as usize][j as usize] = !lights[i as usize][j as usize];
                    }
                }
            }
        }
    }

    let mut count = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if lights[i][j] {
                count += 1;
            }
        }
    }

    count
}


pub fn result_2() -> i64
{    
    let instrs = read_input();
    let mut lights: Vec<Vec<i64>> = vec![vec![0;1000];1000];

    for l in instrs {
        match l.order {
            Order::On => {
                for i in l.start.0..(l.end.0+1) {
                    for j in l.start.1..(l.end.1+1){
                        lights[i as usize][j as usize] += 1;
                    }
                }
            },
            Order::Off => {
                for i in l.start.0..(l.end.0+1) {
                    for j in l.start.1..(l.end.1+1){
                        if lights[i as usize][j as usize] > 0 {
                            lights[i as usize][j as usize] -= 1;
                        }
                    }
                }
            },
            Order::Toggle => {
                for i in l.start.0..(l.end.0+1) {
                    for j in l.start.1..(l.end.1+1){
                        lights[i as usize][j as usize] += 2;
                    }
                }
            }
        }
    }

    let mut count = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            count += lights[i][j];
        }
    }

    count
}

























// I tried to be smart


// see prblm 22 of 2021 
#[allow(dead_code)]
fn count_lights(instrs: &Vec<Instr>, t: bool, rec_count: &mut i64) -> i64 
{
    // println!("{:?}", instrs);
    let mut count = 0;
    *rec_count = *rec_count + 1;

    let len = instrs.len();

    let mut intersections: Vec<Vec<Option<Instr>>> = vec![vec![None; len]; len];

    for i in 0..len {
        for j in (i+1)..len {
            intersections[j][i] = instrs[i].intersection(&instrs[j]);
        }
    }

    for i in 0..len {
        let mut overlaps: Vec<Instr> = vec![];

        for j in 0..i {
            // println!("{:?}", instrs[i].intersection(&instrs[j]));
            if let Some(o) = intersections[i][j] {
                overlaps.push(o);
            }
        }


        if t {
            println!("{i} {} {}", overlaps.len(), rec_count);
            *rec_count = 0;
        }

        let overlap_area = if !overlaps.is_empty() {count_lights(&overlaps, false, rec_count) } else{0};
        // println!("{overlap_area}");

        match instrs[i].order {
            Order::Off => count -= overlap_area,
            Order::On => count = count + instrs[i].area() - overlap_area,
            Order::Toggle => count = count + instrs[i].area() - 2 * overlap_area,
        }
    }

    count
}
