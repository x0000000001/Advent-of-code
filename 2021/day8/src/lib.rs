use std::fs;
use std::collections::HashMap;

const FILE: &str = "input.txt";

fn read_input() -> (Vec<Vec<Vec<char>>>, Vec<Vec<Vec<char>>>) {
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();
    let mut codes: Vec<Vec<Vec<char>>> = Vec::new();

    for line in input{
        let words: Vec<Vec<char>> = line.split_whitespace().map(|el| el.chars().into_iter().collect()).collect();

        patterns.push(words[0..10].to_vec());
        codes.push(words[11..15].to_vec());
    }


    (patterns, codes)
}

pub fn result_1() -> i32
{  
    let (_, codes) = read_input();
    
    let mut count = 0;
    //corresponding to 1,4,7,8
    let possible_unique_vals = [2, 4, 3, 7];

    for list in codes {
        count += list.iter().filter(|el| possible_unique_vals.contains(&el.len())).count();
    }

    count as i32
}

fn decode(patterns: &Vec<Vec<char>>, codes: &Vec<Vec<char>>) -> i32{

    let mut linking: HashMap<char, char> = HashMap::new();

    let seven_pattern = patterns.iter().filter(|el| el.len() == 3).next().unwrap();
    let one_pattern = patterns.iter().filter(|el| el.len() == 2).next().unwrap();
    let four_pattern = patterns.iter().filter(|el| el.len() == 4).next().unwrap();

    let mut to_link: Vec<char>  = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    //used to remove found values from to_link
    let mut last_char_linked: char;

    //a is the only letter in 7 but not 1
    last_char_linked = *seven_pattern.iter().filter(|el| !one_pattern.contains(el)).next().unwrap();
    linking.insert(
        'a', 
        last_char_linked
    );
    to_link.retain(|x| *x != last_char_linked);

    //couting letters apparations
    let mut apparitions: HashMap<char, i32> = HashMap::new();
    for p in patterns {
        for c in p {
            if apparitions.contains_key(c){
                apparitions.insert(*c, apparitions.get(c).unwrap() + 1);
            }else {
                apparitions.insert(*c, 1);
            }
        }
    }

    last_char_linked = *apparitions.iter().filter(|(_,v)| **v == 6).next().unwrap().0;
    //b is the only letter appearing 6 times
    linking.insert(
        'b', 
        last_char_linked
    );
    to_link.retain(|x| *x != last_char_linked);

    last_char_linked = *apparitions.iter().filter(|(_,v)| **v == 4).next().unwrap().0;
    //e is the only letter appearing 4 times
    linking.insert(
        'e', 
        last_char_linked
    );
    to_link.retain(|x| *x != last_char_linked);

    last_char_linked = *apparitions.iter().filter(|(_,v)| **v == 9).next().unwrap().0;
    //f is the only letter appearing 9 times
     linking.insert(
        'f', 
        last_char_linked
    );
    to_link.retain(|x| *x != last_char_linked);

     //c is the only letter appearing 8 times apart from a, that we already have
    last_char_linked = *apparitions.iter().
    filter(|(k,v)| **v == 8 && *k != linking.get(&'a').unwrap())
    .next().unwrap().0;
    linking.insert(
        'c', 
        last_char_linked  
    );
    to_link.retain(|x| *x != last_char_linked);

    //remaining : d and g
    //d appears in 4, not g

    // for l in to_link.iter(){
    //     print!("{l} ");
    // }println!();

    for c in to_link.iter() {
        if four_pattern.contains(&c){
            last_char_linked = *c;
            linking.insert(
                'd', 
                last_char_linked
            );
            break;
        }
    }
    to_link.retain(|x| *x != last_char_linked);

    last_char_linked = to_link[0];
    linking.insert(
        'g', 
        last_char_linked
    );
    to_link.retain(|x| *x != last_char_linked);

    // println!();
    // for (key, val) in linking.iter() {
    //     print!("({} , {})", key, val);
    // }println!();

    //currently, (key, value) pairs are in reversed order to decode
    //we want to invert the dictionnary linking
    let mut invert_linking: HashMap<char, char> = HashMap::new();
    linking.iter().for_each(|(key, value)| {
        invert_linking.insert(*value, *key);
    });

    //stores digits read 
    let mut results = vec![0;4];

    //reading codes
    for i in 0..4 {
        let mut letters = codes[i].clone();

        match letters.len() {
            2 => {results[i] = 1; continue; },
            3 => {results[i] = 7; continue; },
            4 => {results[i] = 4; continue; },
            7 => {results[i] = 8; continue; },
            _ => ()
        }

        //if size isn't enough to decode, reading letters
        for j in 0..letters.len() {
            letters[j] = *invert_linking.get(&letters[j]).unwrap();
        }

        if letters.contains(&'a') && letters.contains(&'b') && letters.contains(&'c') && letters.contains(&'e') && letters.contains(&'f') && letters.contains(&'g'){
            results[i] = 0;
            continue;
        } else if letters.contains(&'a') && letters.contains(&'b')&& letters.contains(&'c') && letters.contains(&'d') && letters.contains(&'f') && letters.contains(&'g'){
            results[i] = 9;
            continue;
        }else if letters.contains(&'a') && letters.contains(&'c') && letters.contains(&'d') && letters.contains(&'e') && letters.contains(&'g'){
            results[i] = 2;
            continue;
        }else if letters.contains(&'a') && letters.contains(&'c') && letters.contains(&'d') && letters.contains(&'f') && letters.contains(&'g'){
            results[i] = 3;
            continue;
        }else if letters.contains(&'a') && letters.contains(&'b') && letters.contains(&'d') && letters.contains(&'e') && letters.contains(&'f') && letters.contains(&'g'){
            results[i] = 6;
            continue;
        }else if letters.contains(&'a') && letters.contains(&'b') && letters.contains(&'d') && letters.contains(&'f') && letters.contains(&'g'){
            results[i] = 5;
            continue;
        }
    }



    let mut sum = 0;
    for n in results {
        sum = sum *10 +n;
    }
    sum
}

pub fn result_2() -> i32
{   
    let (patterns, codes) = read_input();
    
    let mut count: i32 = 0;

    for (p, c) in patterns.iter().zip(codes.iter()) {
        count += decode(p, c);
    }

    count as i32
}