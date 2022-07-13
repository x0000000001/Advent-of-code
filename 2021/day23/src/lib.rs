use std::{fs, fmt::{self}, collections::HashMap};

const FILE: &str = "input.txt";

#[derive(Clone, Eq, Hash, PartialEq)]
struct Burrow {
    // length 11
    corridor: Vec<char>,
    // length 2
    rooms: Vec<Vec<char>>,
    rooms_len: usize
}

const LETTERS: &'static [char] = &['A', 'B', 'C', 'D'];
const ROOMS_POS: &'static [usize] = &[2,4,6,8];

impl Default for Burrow {
    fn default() -> Burrow {
        Burrow {
            corridor: vec!['.'; 11],
            rooms: vec![vec!['.';4];4],
            rooms_len: 4
        }
    }
}


impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display: String = String::default();

        to_display.push_str("#############\n");
        to_display.push_str("#");

        for c in self.corridor.iter() {
            to_display.push(*c);
        }

        to_display.push_str("#\n");

        // line 1 of rooms
        to_display.push_str("###");
        to_display.push(self.rooms[0][0]);
        to_display.push_str("#");

        to_display.push(self.rooms[1][0]);
        to_display.push_str("#");

        to_display.push(self.rooms[2][0]);
        to_display.push_str("#");

        to_display.push(self.rooms[3][0]);
        to_display.push_str("###\n");


        // line 0 of rooms
        for i in 1..self.rooms_len as usize {
            to_display.push_str("  #");
            to_display.push(self.rooms[0][i]);
            to_display.push_str("#");

            to_display.push(self.rooms[1][i]);
            to_display.push_str("#");

            to_display.push(self.rooms[2][i]);
            to_display.push_str("#");

            to_display.push(self.rooms[3][i]);
            to_display.push_str("#\n");
        }

        to_display.push_str("  #########\n");

        write!(f, "{to_display}")
    }
}

impl Burrow {
    #[allow(dead_code)]
    fn count(&self) -> i64
    {
        let mut count = 0;

        for i in 0..4 {
            for j in 0..self.rooms_len {
                if self.rooms[i][j] != '.' {
                    count += 1;
                }
            }
        }

        for i in 0..11 {
            if self.corridor[i] != '.' {
                count += 1;
            }
        }

        count
    }

    fn is_complete(&self) -> bool 
    {
        for i in 0..4 {
            for j in 0..self.rooms_len as usize {
                if self.rooms[i][j] != LETTERS[i] {
                    return false;
                }
            }
        }

        true
    }

    // returns (cost, new_burrow)
    fn available_paths(&self) -> Vec<(i64, Burrow)> 
    {
        let letters_costs: &HashMap<char, i64> = &HashMap::from([('A', 1),('B', 10),('C', 100),('D', 1000)]);
        let rooms_by_letters: &HashMap<char, usize> = &HashMap::from([('A', 0),('B', 1),('C', 2),('D', 3)]);

        let mut paths: Vec<(i64, Burrow)>  = vec![];

        // from corridor to rooms
        let mut foo = |letter_pos: usize| {
            let letter: char = self.corridor[letter_pos];
            let room: usize = *rooms_by_letters.get(&letter).unwrap();
            let room_pos: usize = ROOMS_POS[room];
            let room_index = *rooms_by_letters.get(&letter).unwrap();

            let mut can_go_house = self.rooms[room][0] == '.';
            for i in 1..self.rooms_len as usize {
                if !['.',letter].contains(&self.rooms[room][i]){
                    can_go_house = false;
                }
            }

            if !can_go_house{
                return;
            }

            // is path empty to the room
            for j in room_pos.min(letter_pos)..(room_pos.max(letter_pos)+1) {
                if j == letter_pos{
                    continue;
                }
                if self.corridor[j] != '.' {
                    can_go_house = false;
                    break;
                }
            }
            
            if !can_go_house{
                return;
            }

            // index of letter in its room
            let mut index_in_room = 0;
            while index_in_room < (self.rooms_len - 1) && self.rooms[room][index_in_room + 1] == '.' {
                index_in_room += 1;
            }

            let mut cost = (room_pos as i64 - letter_pos as i64).abs();
            let mut p = self.clone(); 

            cost += index_in_room as i64 + 1;

            p.rooms[room_index][index_in_room] = letter;

            p.corridor[letter_pos] = '.';

            paths.push((cost * letters_costs.get(&letter).unwrap(), p));
        };

        for i in 0..self.corridor.len() {
            let c = self.corridor[i];
            if c != '.' {
                foo(i);
            }
        }

        // from rooms to corridor
        let mut foo1 = |room: usize, letter_pos: usize| {
            let letter = self.rooms[room][letter_pos];
            let room_pos = ROOMS_POS[room];

            let mut start_index = room_pos;
            while start_index > 0 && self.corridor[start_index - 1] == '.' {
                start_index -= 1;
            }

            let mut end_index = room_pos;
            while end_index < 11 && self.corridor[end_index] == '.' {
                end_index += 1;
            }

            for i in start_index..end_index {
                //can't stop in front of a room
                if [2,4,6,8].contains(&i) {
                    continue;
                }

                let cost = (i as i64 - room_pos as i64).abs() + letter_pos as i64 + 1;
                let mut p = self.clone();

                p.rooms[room][letter_pos] = '.';
                p.corridor[i] = letter;

                paths.push((cost * letters_costs.get(&letter).unwrap(), p));
            }
        };

        for room in 0..4 {
            let letter = LETTERS[room];
            for i in 0..self.rooms_len {
                if self.rooms[room][i] != '.' {
                    // not letting letters go out if everything is okay
                    let mut can_go_out: bool = self.rooms[room][i] != letter;

                    if !can_go_out {
                        for j in (i+1)..self.rooms_len{
                            if self.rooms[room][j] != letter {
                                can_go_out = true;
                                break;
                            }
                        }
                    }

                    if can_go_out {
                        foo1(room, i);
                    }

                    break;
                }
            }
        }
        
        paths
    }
}

fn read_input() -> Burrow
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();
   
    let mut b = Burrow::default();

    b.rooms[0][0] = input[2].chars().nth(3).unwrap();
    b.rooms[1][0] = input[2].chars().nth(5).unwrap();
    b.rooms[2][0] = input[2].chars().nth(7).unwrap();
    b.rooms[3][0] = input[2].chars().nth(9).unwrap();

    b.rooms_len = input.len() - 3;

    for i in 3..(b.rooms_len + 2){
        b.rooms[0][i-2] = input[i].chars().nth(1).unwrap();
        b.rooms[1][i-2] = input[i].chars().nth(3).unwrap();
        b.rooms[2][i-2] = input[i].chars().nth(5).unwrap();
        b.rooms[3][i-2] = input[i].chars().nth(7).unwrap();
    }

    b
}

fn find_min_cost(burrow: &Burrow, already_seen: &mut HashMap<Burrow,Option<i64>>) -> Option<i64>
{
    // println!("{}", burrow);

    if burrow.is_complete() {
        return Some(0);
    }else{
        let mut min_cost = None;
        let mut paths = burrow.available_paths();

        // if one path ends, the others won't be better
        for (c,b) in paths.iter() {
            if b.is_complete() {
                paths = vec![(*c,b.clone());1];
                break;
            }
        }

        for (c, b) in paths
        {
            if !already_seen.contains_key(&b){
                let new_cost = find_min_cost(&b, already_seen);
                already_seen.insert(b.clone(), new_cost);
            }

            if let Some(end_cost) = already_seen.get(&b).unwrap(){
                let candidate_cost = c + end_cost;
                if min_cost.is_none() || candidate_cost < min_cost.unwrap() {
                    min_cost = Some(candidate_cost);
                }
            }
        }

        min_cost
    }
}

pub fn result_1() -> i64
{
    let input = read_input();
    // input.corridor[0] = 'C';
    // input.corridor[10] = 'C';
    // println!("{}", input);
    // for (c,b) in input.available_paths() {
    //     println!("{c}   {}\n{b}", b.is_complete());
    // }
    // 0

    find_min_cost(&input, &mut HashMap::new()).unwrap()
}


pub fn result_2() -> i64
{   
    let mut input = read_input();
    
    input.rooms_len = 4;
    for i in 0..4 {
        input.rooms[i][3] = input.rooms[i][1];
    }
    input.rooms[0][1] = 'D';
    input.rooms[1][1] = 'C';
    input.rooms[2][1] = 'B';
    input.rooms[3][1] = 'A';

    input.rooms[0][2] = 'D';
    input.rooms[1][2] = 'B';
    input.rooms[2][2] = 'A';
    input.rooms[3][2] = 'C';


    find_min_cost(&input, &mut HashMap::new()).unwrap()
}