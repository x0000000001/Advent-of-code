use std::fs;

pub fn result_1() -> i32
{   
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
        
    let mut hoz = 0;
    let mut depth = 0;

    for line in contents.lines() {
        let mut els = line.split_whitespace();

        let instruction = els.next().unwrap();
        let amplitude:i32 = els.next().unwrap().parse().unwrap();

        match instruction {
            "forward"=>hoz+=amplitude,
            "up"=>depth-=amplitude,
            "down"=>depth+=amplitude,
            _=> () ,
        }
    }

    hoz*depth
}

pub fn result_2() -> i32
{   
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
        
    let mut hoz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in contents.lines() {
        let mut els = line.split_whitespace();

        let instruction = els.next().unwrap();
        let amplitude:i32 = els.next().unwrap().parse().unwrap();

        match instruction {
            "forward"=>{
                hoz+=amplitude; 
                depth += aim * amplitude;
            },
            "up"=>aim-=amplitude,
            "down"=>aim+=amplitude,
            _=> () ,
        }
    }

    hoz*depth
}