use std::collections::HashMap;

pub type InputType = Vec<(i64,i64,i64,i64,i64,i64)>;

// 0 => value goes to bot (0,bot,value,0,0,0)
// 1 => low goes to, high goes to (1,bot,destination_low,destination_high,is_bot,is_bot)
// -1 => is output, 1 => is bot
pub fn result_1(mut input: InputType) -> i64
{
    let mut outputs: HashMap<i64,i64> = HashMap::new();
    let mut bots: HashMap<i64,Vec<i64>> = HashMap::new();

    while !input.is_empty() {
        
        let mut to_remove: Vec<usize> = vec![];
        for i in 0..input.len(){
            let (instr, bot,n0,n1,is_bot0,is_bot1) = input[i];
            
            if instr == 0 {
                let accessor = bots.entry(bot.clone()).or_insert(vec![]);
                if accessor.len() < 2 {
                    accessor.push(n0);
                    to_remove.push(i);
                }
            }
            else if instr == 1 {
                let accessor = bots.entry(bot.clone()).or_insert(vec![]);
                if accessor.len() < 2 {
                    continue;
                }
                accessor.sort();
                let (x,y) = (accessor[0],accessor[1]);

                if x == 17 && y == 61 {
                    return bot;
                }

                if is_bot0 == -1 {
                    let accessor1 = outputs.entry(n0).or_insert(0);
                    *accessor1 = x;
                } else {
                    let accessor1 = bots.entry(n0).or_insert(vec![]);
                    accessor1.push(x);
                }

                if is_bot1 == -1 {
                    let accessor1 = outputs.entry(n1).or_insert(0);
                    *accessor1 = y;
                } else {
                    let accessor1 = bots.entry(n1).or_insert(vec![]);
                    accessor1.push(y)
                }
                to_remove.push(i);
            }
        }

        for i in to_remove.into_iter().rev() {
            input.remove(i);
        }
    }

    -1
}


pub fn result_2(mut input: InputType) -> i64
{ 
    let mut outputs: HashMap<i64,i64> = HashMap::new();
    let mut bots: HashMap<i64,Vec<i64>> = HashMap::new();

    while !input.is_empty() {
        
        let mut to_remove: Vec<usize> = vec![];
        for i in 0..input.len(){
            let (instr, bot,n0,n1,is_bot0,is_bot1) = input[i];
            
            if instr == 0 {
                let accessor = bots.entry(bot.clone()).or_insert(vec![]);
                if accessor.len() < 2 {
                    accessor.push(n0);
                    to_remove.push(i);
                }
            }
            else if instr == 1 {
                let accessor = bots.entry(bot.clone()).or_insert(vec![]);
                if accessor.len() < 2 {
                    continue;
                }
                accessor.sort();
                let (x,y) = (accessor[0],accessor[1]);

                if is_bot0 == -1 {
                    let accessor1 = outputs.entry(n0).or_insert(0);
                    *accessor1 = x;
                } else {
                    let accessor1 = bots.entry(n0).or_insert(vec![]);
                    accessor1.push(x);
                }

                if is_bot1 == -1 {
                    let accessor1 = outputs.entry(n1).or_insert(0);
                    *accessor1 = y;
                } else {
                    let accessor1 = bots.entry(n1).or_insert(vec![]);
                    accessor1.push(y)
                }
                to_remove.push(i);
            }
        }

        for i in to_remove.into_iter().rev() {
            input.remove(i);
        }
    }

    if let Some(m) = outputs.get(&0) {
        if let Some(n) = outputs.get(&1) {
            if let Some(p) = outputs.get(&2) {
                return m*n*p;
            }
        }
    }

    -1
}