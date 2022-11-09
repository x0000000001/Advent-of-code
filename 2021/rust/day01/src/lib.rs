use std::fs;

pub fn result_1() -> i32
{   
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut x = 0;
    let mut temp: i32 = contents.lines().next().unwrap().parse().unwrap();

    let mut it = contents.lines();
    it.next();

    for line in it
    {
        let value:i32 = line.parse().unwrap();
        if value > temp  {
            x+=1;
        }
        temp = value;
    }

    x
}

pub fn result_2() -> i32
{   
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut x = 0;
    let mut vals = [0;3];

    let mut it = contents.lines();

    for i in 0..3{
        vals[i] = it.next().unwrap().parse().unwrap();
    }

    let mut previous_sum: i32 = vals.iter().sum();

    for line in it
    {
        for i in 0..2{
            vals[i] = vals[i+1];
        }
        
        vals[2]= line.parse().unwrap();

        let new_sum:i32 = vals.iter().sum();
        
        if new_sum > previous_sum{
            x+=1;
        }
        previous_sum = new_sum;
    }

    x
}