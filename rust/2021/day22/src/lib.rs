use std::{fs};

const FILE: &str = "input.txt";

#[derive(Default, Clone, Copy, Debug)]
struct Cuboïd {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
    z0: i64,
    z1: i64
}

fn read_input() -> Vec<(bool, Cuboïd)>
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();
    let mut steps: Vec<(bool, Cuboïd)> = vec![];

    for l in input.iter(){
        let mut temp: String = l.chars().filter(|c| [',','.','-','0','1','2','3','4','5','6','7','8','9'].contains(c)).collect();
        temp = temp.replace(",", "..");
        let nums = temp.split("..").map(|el| el.parse().unwrap()).collect::<Vec<i64>>();

        let b: bool = &l[0..2] == "on";

        steps.push((b,Cuboïd{ x0: nums[0], x1: nums[1], y0: nums[2], y1: nums[3], z0: nums[4], z1: nums[5]}));
    }

    steps
}

impl Cuboïd {
// returns intersection between two cuboïds, if there's any

    fn intersection(&self, c1: Cuboïd) -> Option<  Cuboïd>
    {
        if self.x0 > c1.x1 || self.x1 < c1.x0 || self.y0 > c1.y1 || self.y1 < c1.y0 || self.z0 > c1.z1 || self.z1 < c1.z0 {
            return None;
        }else{
            let mut c = Cuboïd::default();

            c.x0 = self.x0.max(c1.x0);
            c.x1 = self.x1.min(c1.x1);
            c.y0 = self.y0.max(c1.y0);
            c.y1 = self.y1.min(c1.y1);
            c.z0 = self.z0.max(c1.z0);
            c.z1 = self.z1.min(c1.z1);

            Some(c)
        }
    } 

    fn volume(&self) -> i64
    {
        (self.x1 - self.x0 + 1) * (self.y1 - self.y0 + 1) * (self.z1 - self.z0 + 1)
    }

    fn max_amplitude(&self) -> i64 {
        return self.x0.abs().max(self.x1.abs().max(self.y0.abs().max(self.y1.abs().max(self.z0.abs().max(self.z1.abs())))));
    }
}

/// counts on blocks after all moves applied on a n x n x n grid
fn count_blocks(moves: &Vec<(bool, Cuboïd)>) -> i64 {
    let len = moves.len();
    // matrix of intersection bewteen moves
    let mut intersections: Vec<Vec<Option<Cuboïd>>> = vec![vec![None; len]; len];

    for i in 0..len {
        for j in (i+1)..len {
            intersections[j][i] = moves[i].1.intersection(moves[j].1);
        }
    }

    let mut blocks_count = 0;
    
    for i in 0..len {
        let (b,cube) = moves[i];

        // !!!!!!! TRICK IS HERE  !!!!!!!
        // we determine recursively the intersection zone of each move with previous moves
        let mut new_moves: Vec<(bool, Cuboïd)> = vec![];
        for j in 0..i {
            if let Some(c) = intersections[i][j] {
                new_moves.push((moves[j].0, c));
            }
        }

        let intersect_volume = count_blocks(&new_moves);

        if b {
            blocks_count += cube.volume();
            blocks_count -= intersect_volume;
        }else {
            blocks_count -= intersect_volume;
        }

        // end of trick zone
    }

    blocks_count
}

pub fn result_1() -> i64
{
    let input = read_input();
    let acceptable_moves: Vec<(bool, Cuboïd)> = input.into_iter().filter(|c| c.1.max_amplitude() <= 50).collect();

    count_blocks(&acceptable_moves)
}


pub fn result_2() -> i64
{   
    let input = read_input();
    count_blocks(&input)
}