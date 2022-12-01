pub type InputType = Vec<[i64;5]>;

pub fn score1(quantities: &Vec<i64>, ing: &Vec<[i64;5]>) -> i64 {
    let mut s = 1;

    for j in 0..4 {
        let mut temp = 0;
        for i in 0..ing.len() {
            temp += quantities[i] * ing[i][j];
        }
        if temp <= 0 {
            return 0;
        }
        s *= temp;
    }

    s
}

/// Brute-force solution. \
/// I don't like it, but I didn't have the motivation to learn
/// a new constraint solver protocol just for this. \ 
/// Especially since in Rust I didn't find any really famous/ easy-to-use. 
pub fn result_1(ing: InputType) -> i64
{
    let mut max = 0;
    for i in 0..100 {
        for j in 0..(100-i) {
            for k in 0..(100-i-j) {
                let l = 100-i-j-k;
                let score = score1(&Vec::from([i,j,k,l]), &ing);
                if score > max {
                    max = score;
                }
            }
        }
    }

    max
}


pub fn result_2(ing: InputType) -> i64
{   
    let mut max = 0;
    for i in 0..100 {
        for j in 0..(100-i) {
            for k in 0..(100-i-j) {
                let l = 100-i-j-k;
                let score = score1(&Vec::from([i,j,k,l]), &ing);
                let cals = i * ing[0][4] + j * ing[1][4] + k * ing[2][4] + l * ing[3][4];
                
                if cals != 500 {
                    continue;
                }
                
                if score > max {
                    max = score;
                }
            }
        }
    }

    max
}