pub type InputType = Vec<(i64,usize,usize)>;

fn print_grid(g: &Vec<Vec<bool>>) {
    for j in 0..6 {
        for i in 0..50 {
            print!("{}", if g[i][j] {"#"} else {"."});
        }
        println!();
    }
}

fn solve(input: InputType) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = vec![vec![false;6];50];

    for (order,n0,mut n1) in input {
        match order {
            0 => {
                for i in 0..n0 {
                    for j in 0..n1 {
                        grid[i][j] = true;
                    }
                }
            },
            1 => {
                let temp: Vec<bool> = (0..50).map(|i| grid[i][n0]).collect();
                n1 = n1%50;
                for i in 0..50 {
                    let index = if n1 > i {50-n1+i} else {i-n1};
                    grid[i][n0] = temp[index];
                }
            },
            _ => {
                let temp: Vec<bool> = grid[n0].clone();
                n1 = n1%6;
                for j in 0..6 {
                    let index = if n1>j{6-n1+j} else {j-n1};
                    grid[n0][j] = temp[index];
                }
            }
        }
    }

    grid
}

//0 = rect, 1 = row, 2 = column
pub fn result_1(input: InputType) -> i64
{
    solve(input).into_iter().flat_map(|l| l.into_iter().map(|c| if c {1} else {0})).sum()
}

/// Not numerical, read output.
pub fn result_2(input: InputType) -> i64
{    
    print_grid(&mut solve(input));
    0
}