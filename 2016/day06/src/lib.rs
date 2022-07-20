use std::collections::HashMap;

pub type InputType = Vec<Vec<char>>;

pub fn result_1(input: InputType) -> i64
{
    let len = input[0].len();
    let mut counts: Vec<HashMap<char,u64>> = vec![HashMap::new(); len];

    for l in input {
        for i in 0..len {
            let accessor = counts[i].entry(l[i]).or_insert(0);
            *accessor += 1;
        }
    }

    let res = counts.into_iter().map(|h| {
        let mut c = h.into_iter().collect::<Vec<(char,u64)>>();
        c.sort_by(|(_,c0),(_,c1)| c0.cmp(c1));
        c.last().unwrap().0.clone()
    }).collect::<String>();

    println!("{}",res);

    0
}


pub fn result_2(input: InputType) -> i64
{   
    let len = input[0].len();
    let mut counts: Vec<HashMap<char,u64>> = vec![HashMap::new(); len];

    for l in input {
        for i in 0..len {
            let accessor = counts[i].entry(l[i]).or_insert(0);
            *accessor += 1;
        }
    }

    let res = counts.into_iter().map(|h| {
        let mut c = h.into_iter().collect::<Vec<(char,u64)>>();
        c.sort_by(|(_,c0),(_,c1)| c0.cmp(c1));
        c.first().unwrap().0.clone()
    }).collect::<String>();

    println!("{}",res);

    0
}