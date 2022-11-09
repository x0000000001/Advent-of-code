pub type InputType = String;

const OPEN_LETTERS: &[char] = &['b','c','d','e','f'];

fn get_open_doors(s: &str, (x,y): (usize,usize)) -> Vec<char> {
    let mut open_doors = vec![];

    let hash = format!("{:x}",md5::compute(s)).chars().collect::<Vec<char>>();
    if y > 0 && OPEN_LETTERS.contains(&hash[0]) {
        open_doors.push('U');
    }
    if y < 3 && OPEN_LETTERS.contains(&hash[1]) {
        open_doors.push('D');
    }
    if x > 0 && OPEN_LETTERS.contains(&hash[2]) {
        open_doors.push('L');
    }
    if x < 3 && OPEN_LETTERS.contains(&hash[3]) {
        open_doors.push('R');
    }


    open_doors
}

/// Read printed output.
pub fn result_1(password: InputType) -> i64
{   
    // path is id, which len serves as cost
    // second arguement is current position
    let mut queue: Vec<(String,(usize,usize))> = vec![("".to_string(),(0,0));1];

    while !queue.is_empty() {
        queue.sort_by(|(s0,_),(s1,_)| s1.len().cmp(&s0.len()));
        let (path,coord) = queue.pop().unwrap();

        if coord == (3,3) {
            println!("{path}");
            return 0;
        }

        let open_doors = get_open_doors(&format!("{password}{}",path),coord);

        for door in open_doors {
            let new_coord = match door {
                'L' => (coord.0-1,coord.1),
                'R' => (coord.0+1,coord.1),
                'U' => (coord.0,coord.1-1),
                'D' => (coord.0,coord.1+1),
                _ => panic!()
            };

            let new_path = format!("{}{}",path,door);

            queue.push((new_path,new_coord));
        }
    }

    1
}

pub fn result_2(password: InputType) -> i64
{   
     
    // path is id, which len serves as cost
    // second arguement is current position
    let mut queue: Vec<(String,(usize,usize))> = vec![("".to_string(),(0,0));1];
    let mut max_length = 0;

    while !queue.is_empty() {
        queue.sort_by(|(s0,_),(s1,_)| s1.len().cmp(&s0.len()));
        let (path,coord) = queue.pop().unwrap();


        if coord == (3,3) {
            max_length = max_length.max(path.len());
            continue;
        }

        let open_doors = get_open_doors(&format!("{password}{}",path),coord);

        for door in open_doors {
            let new_coord = match door {
                'L' => (coord.0-1,coord.1),
                'R' => (coord.0+1,coord.1),
                'U' => (coord.0,coord.1-1),
                'D' => (coord.0,coord.1+1),
                _ => panic!()
            };

            let new_path = format!("{}{}",path,door);

            queue.push((new_path,new_coord));
        }
    }

    max_length as i64
}