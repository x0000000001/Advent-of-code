pub type InputType = Vec<bool>;

fn count_safe_tiles(mut row: InputType, rows_count: usize) -> u64 {
    let mut safe_tiles_count = 0;
    
    for _ in 0..rows_count {
        safe_tiles_count += row.iter().filter(|b| !*b).count();

        let mut temp = vec![];

        for i in 0..row.len() {
            let previous = [
                if i > 0 {row[i-1]} else {false},
                row[i],
                if i +1 < row.len() {row[i+1]} else {false}
            ];

            temp.push(match previous {
                [true,true,false] => true,
                [false,true,true] => true,
                [true,false,false] => true,
                [false,false,true] => true,
                _ => false
            });
        }

        row.clear();
        row.append(&mut temp);
    }


    safe_tiles_count as u64
}

pub fn result_1(row: InputType) -> i64
{
    count_safe_tiles(row, 40) as i64
}


pub fn result_2(row: InputType) -> i64
{   
    count_safe_tiles(row, 400000) as i64
}