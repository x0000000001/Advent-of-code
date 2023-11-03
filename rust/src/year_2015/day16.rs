use crate::Solution;

pub type InputType = Vec<AuntSue>;

#[derive(Debug)]
pub struct AuntSue {
    // -1 means unknown
    pub children: i64,
    pub cats: i64,
    pub samoyeds: i64,
    pub pomeranians: i64,
    pub akitas: i64,
    pub vizslas: i64,
    pub goldfish: i64,
    pub trees: i64,
    pub cars: i64,
    pub perfumes: i64,
}

impl Default for AuntSue {
    fn default() -> Self {
        Self {
            children: -1,
            cats: -1,
            samoyeds: -1,
            pomeranians: -1,
            akitas: -1,
            vizslas: -1,
            goldfish: -1,
            trees: -1,
            cars: -1,
            perfumes: -1,
        }
    }
}

const RIGHT_SUE: AuntSue = AuntSue {
    children: 3,
    cats: 7,
    samoyeds: 2,
    pomeranians: 3,
    akitas: 0,
    vizslas: 0,
    goldfish: 5,
    trees: 3,
    cars: 2,
    perfumes: 1,
};

pub fn part1(s: String) -> Solution {
    let aunts = parse(s);

    let mut i = 0;
    for a in aunts {
        i += 1;
        if a.children != -1 && a.children != RIGHT_SUE.children {
            continue;
        }
        if a.cats != -1 && a.cats != RIGHT_SUE.cats {
            continue;
        }
        if a.samoyeds != -1 && a.samoyeds != RIGHT_SUE.samoyeds {
            continue;
        }
        if a.pomeranians != -1 && a.pomeranians != RIGHT_SUE.pomeranians {
            continue;
        }
        if a.akitas != -1 && a.akitas != RIGHT_SUE.akitas {
            continue;
        }
        if a.vizslas != -1 && a.vizslas != RIGHT_SUE.vizslas {
            continue;
        }
        if a.goldfish != -1 && a.goldfish != RIGHT_SUE.goldfish {
            continue;
        }
        if a.trees != -1 && a.trees != RIGHT_SUE.trees {
            continue;
        }
        if a.cars != -1 && a.cars != RIGHT_SUE.cars {
            continue;
        }
        if a.perfumes != -1 && a.perfumes != RIGHT_SUE.perfumes {
            continue;
        }
        break;
    }

    Solution::from(i)
}

pub fn part2(s: String) -> Solution {
    let aunts = parse(s);

    let mut i = 0;
    for a in aunts {
        i += 1;
        if a.children != -1 && a.children != RIGHT_SUE.children {
            continue;
        }
        if a.cats != -1 && a.cats <= RIGHT_SUE.cats {
            continue;
        }
        if a.samoyeds != -1 && a.samoyeds != RIGHT_SUE.samoyeds {
            continue;
        }
        if a.pomeranians != -1 && a.pomeranians >= RIGHT_SUE.pomeranians {
            continue;
        }
        if a.akitas != -1 && a.akitas != RIGHT_SUE.akitas {
            continue;
        }
        if a.vizslas != -1 && a.vizslas != RIGHT_SUE.vizslas {
            continue;
        }
        if a.goldfish != -1 && a.goldfish >= RIGHT_SUE.goldfish {
            continue;
        }
        if a.trees != -1 && a.trees <= RIGHT_SUE.trees {
            continue;
        }
        if a.cars != -1 && a.cars != RIGHT_SUE.cars {
            continue;
        }
        if a.perfumes != -1 && a.perfumes != RIGHT_SUE.perfumes {
            continue;
        }
        break;
    }

    Solution::from(i)
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut aunts: InputType = vec![];

    for l in input {
        let l = l.replace(",", "");
        let l = l.replace(":", "");
        let mut words = l.split_whitespace();
        words.next();
        words.next();

        let mut next_aunt = AuntSue::default();

        while let Some(selector) = words.next() {
            let count = words.next().unwrap().parse::<i64>().unwrap();

            match selector {
                "children" => next_aunt.children = count,
                "cats" => next_aunt.cats = count,
                "samoyeds" => next_aunt.samoyeds = count,
                "pomeranians" => next_aunt.pomeranians = count,
                "akitas" => next_aunt.akitas = count,
                "vizslas" => next_aunt.vizslas = count,
                "goldfish" => next_aunt.goldfish = count,
                "trees" => next_aunt.trees = count,
                "cars" => next_aunt.cars = count,
                "perfumes" => next_aunt.perfumes = count,
                _ => (),
            }
        }

        aunts.push(next_aunt);
    }

    aunts
}
