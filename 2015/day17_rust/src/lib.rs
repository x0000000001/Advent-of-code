pub type InputType = Vec<u64>;

pub fn count_ways(n: u64, capacities: &Vec<u64>) -> u64{
    let mut total = 0;
    
    for i in 0..capacities.len(){
        let el = capacities[i];
        total += 
            if n == el {
                1
            } else if n > el {
                let val = n - el;

                let mut new_capacities = vec![];
                for j in (i+1)..capacities.len() {
                    new_capacities.push(capacities[j]);
                }

                count_ways(val, &new_capacities)
                
            } else { 0 }
    };

    total
}

pub fn result_1(input: InputType) -> i64
{
    count_ways(150, &input) as i64
}

// returns : min recipients used, nb of combinations for that min
pub fn count_min_ways(n: u64, capacities: &Vec<u64>) -> (u64, u64){
    let mut total = 0;
    let mut min = u64::MAX;
    
    for i in 0..capacities.len(){
        let el = capacities[i];

        if n == el {
            if min > 0 {
                min = 0;
                total = 0;
            }
            total += 1;
        } else if n > el {
            let val = n - el;

            let mut new_capacities = vec![];
            for j in (i+1)..capacities.len() {
                new_capacities.push(capacities[j]);
            }

            let (sub_min, sub_total) = count_min_ways(val, &new_capacities);
            if sub_min < min {
                min = sub_min;
                total = sub_total;
            } else if sub_min == min {
                total += sub_total;
            }
        }
    };
    
    if min != u64::MAX {
        min += 1;
    }
    (min, total)
}

pub fn result_2(input: InputType) -> i64
{   
    
    count_min_ways(150, &input).1 as i64
}