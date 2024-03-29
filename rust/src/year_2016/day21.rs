use crate::Solution;

type InputType = Vec<Box<dyn Instr>>;

pub trait Instr {
    fn apply(&self, s: &mut Vec<char>);
    fn unapply(&self, s: &mut Vec<char>);
}

struct SwapPos {
    x: usize,
    y: usize,
}

impl Instr for SwapPos {
    fn apply(&self, s: &mut Vec<char>) {
        s.swap(self.x, self.y);
    }

    fn unapply(&self, s: &mut Vec<char>) {
        self.apply(s);
    }
}

struct SwapLetter {
    x: char,
    y: char,
}

impl Instr for SwapLetter {
    fn apply(&self, s: &mut Vec<char>) {
        let x = s.iter().position(|c| *c == self.x).unwrap();
        let y = s.iter().position(|c| *c == self.y).unwrap();
        s.swap(x, y);
    }

    fn unapply(&self, s: &mut Vec<char>) {
        self.apply(s);
    }
}

struct RotateLR {
    to_the_right: bool,
    steps: usize,
}

impl Instr for RotateLR {
    fn apply(&self, s: &mut Vec<char>) {
        if self.to_the_right {
            s.rotate_right(self.steps);
        } else {
            s.rotate_left(self.steps);
        }
    }

    fn unapply(&self, s: &mut Vec<char>) {
        if !self.to_the_right {
            s.rotate_right(self.steps);
        } else {
            s.rotate_left(self.steps);
        }
    }
}

struct RotatePos {
    x: char,
}

impl Instr for RotatePos {
    fn apply(&self, s: &mut Vec<char>) {
        let x = s.iter().position(|c| *c == self.x).unwrap();
        let bonus = if x > 3 { 1 } else { 0 };
        let len = s.len();
        s.rotate_right((x + 1 + bonus) % len);
    }

    /// The naïve way : trying every rotation until finding the one that works.
    fn unapply(&self, s: &mut Vec<char>) {
        for i in 0..(s.len()) {
            let mut sp = s.clone();
            sp.rotate_left(i);
            self.apply(&mut sp);
            if sp == *s {
                s.rotate_left(i);
                return;
            }
        }
    }
}

struct ReversePos {
    x: usize,
    y: usize,
}

impl Instr for ReversePos {
    fn apply(&self, s: &mut Vec<char>) {
        let s_lice = &mut s[self.x..(self.y + 1)];
        s_lice.reverse();
    }

    fn unapply(&self, s: &mut Vec<char>) {
        self.apply(s);
    }
}

struct MovePos {
    x: usize,
    y: usize,
}

impl Instr for MovePos {
    fn apply(&self, s: &mut Vec<char>) {
        let c = s.remove(self.x);
        s.insert(self.y, c);
    }

    fn unapply(&self, s: &mut Vec<char>) {
        let c = s.remove(self.y);
        s.insert(self.x, c);
    }
}

fn scramble(input: InputType, s: &mut Vec<char>) {
    for instr in input {
        instr.apply(s);
    }
}

fn unscramble(mut input: InputType, s: &mut Vec<char>) {
    input.reverse();
    for instr in input {
        instr.unapply(s);
    }
}

pub fn part1(s: String) -> Solution {
    let input = parse(s);

    let mut s = "abcdefgh".chars().collect();
    scramble(input, &mut s);
    Solution::from(s.iter().collect::<String>())
}

pub fn part2(s: String) -> Solution {
    let input = parse(s);

    let mut s = "fbgdceah".chars().collect();
    unscramble(input, &mut s);
    Solution::from(s.iter().collect::<String>())
}

fn parse(s: String) -> InputType {
    let input: Vec<String> = s
        .lines()
        .into_iter()
        .map(|line| line.trim().to_owned())
        .collect();

    let mut res: InputType = vec![];

    for l in input {
        let w = l.split_whitespace().collect::<Vec<&str>>();
        match (w[0], w[1]) {
            ("swap", "position") => res.push(Box::new(SwapPos {
                x: w[2].parse().unwrap(),
                y: w.last().unwrap().parse().unwrap(),
            })),

            ("swap", "letter") => res.push(Box::new(SwapLetter {
                x: w[2].chars().next().unwrap(),
                y: w.last().unwrap().chars().next().unwrap(),
            })),

            ("rotate", "left") => res.push(Box::new(RotateLR {
                to_the_right: false,
                steps: w[2].parse().unwrap(),
            })),

            ("rotate", "right") => res.push(Box::new(RotateLR {
                to_the_right: true,
                steps: w[2].parse().unwrap(),
            })),

            ("rotate", "based") => res.push(Box::new(RotatePos {
                x: w.last().unwrap().chars().next().unwrap(),
            })),

            ("reverse", "positions") => res.push(Box::new(ReversePos {
                x: w[2].parse().unwrap(),
                y: w.last().unwrap().parse().unwrap(),
            })),

            ("move", "position") => res.push(Box::new(MovePos {
                x: w[2].parse().unwrap(),
                y: w.last().unwrap().parse().unwrap(),
            })),

            _ => panic!(),
        }
    }

    res
}
