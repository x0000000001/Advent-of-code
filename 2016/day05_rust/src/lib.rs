pub type InputType = String;

/// 1 min to run. \
/// Result is not numerical : read output.
pub fn result_1(input: InputType) -> i64
{
    let mut n: Vec<char> = vec![];
    let mut count = 0;
    let mut i = 0;
    
    while count < 8 {
        let digest = md5::compute(format!("{input}{}", i.to_string()));
        let result = format!("{:x}", digest);
        if &result[0..5] == "00000" {
            println!("{i}");
            n.push(result.chars().nth(5).unwrap());
            count += 1;
        }

        i+=1;
    }

    println!("{}", n.iter().collect::<String>());

    0
}

/// 2 min to run. \
/// Result is not numerical : read output.
pub fn result_2(input: InputType) -> i64
{   
    let mut n: Vec<Option<char>> = vec![None;8];
    let mut count = 0;
    let mut i = 0;
    
    while count < 8 {
        let digest = md5::compute(format!("{input}{}", i.to_string()));
        let result = format!("{:x}", digest);
        if &result[0..5] == "00000" {
            if let Some(index) = result.chars().nth(5).unwrap().to_digit(10) {
                if index < 8 && n[index as usize].is_none() {
                    println!("{i}");
                    n[index as usize] = Some(result.chars().nth(6).unwrap());
                    count += 1;
                }
            }
        }

        i+=1;
    }

    println!("{}", n.iter().map(|el| el.unwrap()).collect::<String>());

    0
}