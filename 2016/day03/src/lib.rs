pub type InputType = Vec<Vec<i64>>;

pub fn result_1(input: InputType) -> i64
{
    input.into_iter().map(|mut t| {t.sort(); t}).filter(|t| t[0]+t[1] > t[2]).count() as i64
}


pub fn result_2(input: InputType) -> i64
{   
    let mut nt: Vec<Vec<i64>> = vec![];

    for i in 0..input.len()/3 {
        nt.push(Vec::from([input[3*i][0],input[3*i+1][0],input[3*i+2][0]]));
        nt.push(Vec::from([input[3*i][1],input[3*i+1][1],input[3*i+2][1]]));
        nt.push(Vec::from([input[3*i][2],input[3*i+1][2],input[3*i+2][2]]));
    }
    
    nt.into_iter().map(|mut t| {t.sort(); t}).filter(|t| t[0]+t[1] > t[2]).count() as i64
}