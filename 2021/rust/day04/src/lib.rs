use std::fs;

const FILE: &str = "input.txt";

fn read_input() -> (Vec<i32>, Vec<Vec<Vec<(bool,i32)>>>)
{

    let contents = fs::read_to_string(FILE)
        .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let chosen_numbers: Vec<i32> = input[0].split(',').map(|num| num.parse().unwrap()).collect();

    //counting boards
    let boards_count = input.iter().filter(|line| line.is_empty()).count();
    
    let mut boards:Vec<Vec<Vec<(bool,i32)>>> = vec![vec![vec![(false,0);5];5];boards_count];

    let mut line_index = 2;
    let mut board_index = 0;
    while line_index + 1 < input.len() {
        for j in 0..5 {
            boards[board_index][j] = input[line_index + j].split_whitespace().map(|str| (false, str.parse().unwrap())).collect();
        }

        line_index += 6;
        board_index += 1;
    }

    (chosen_numbers, boards)
}

//returns the sum of all unmarked value in grid
fn sum_of_grid(grid: &Vec<Vec<(bool,i32)>>) -> i32 {
    let mut sum = 0;
    for i in 0..5{
        for j in 0..5{
            if !grid[i][j].0 {
                sum += grid[i][j].1;
            }
        }
    }
    sum
}


//returns -1 if grid is invalid, the grid score otherwise
fn is_grid_valid(grid: &Vec<Vec<(bool,i32)>>) -> i32{
    for i in 0..5 {
        //checking lines
        let mut is_valid = true;
        for j in 0..5 {
            let (is_checked, _) = grid[i][j];
            if !is_checked { is_valid = false; break;}
        }
        if is_valid {
            return sum_of_grid(grid);
        }

        //checking columns
        let mut is_valid = true;
        for j in 0..5 {
            let (is_checked, _) = grid[j][i];
            if !is_checked { is_valid = false; break;}
        }
        if is_valid {
            return sum_of_grid(grid);
        }
    }

    -1
}

pub fn result_1() -> i32
{  
    let (chosen_numbers, mut boards) = read_input();

    for number in chosen_numbers {
        for k in 0..boards.len() {
            for i in 0..5 {
                for j in 0..5 {
                    if boards[k][i][j].1 == number {
                        boards[k][i][j].0 = true;
                    }
                } 
            }
        }

        for b in boards.iter() {
            let val = is_grid_valid(b);
            if val != -1 {
                return val * number;
            }
        }
    }

    0
}

fn print_grid(grid: &Vec<Vec<(bool,i32)>>){
    for line in grid.iter() {
        for el in line{
            print!("{} ", el.1);
        }

        println!();
    }
}

pub fn result_2() -> i32
{   
    let (chosen_numbers, boards_temp) = read_input();
    let mut boards: Vec<(bool, Vec<Vec<(bool, i32)>>)> = boards_temp.iter().map(|b| (false, b.clone())).collect();

    //used to return only when the last grid has won
    let mut only_1_remaining = false;

    for number in chosen_numbers {

        for k in 0..boards.len() {
            for i in 0..5 {
                for j in 0..5 {
                    if boards[k].1[i][j].1 == number {
                        boards[k].1[i][j].0 = true;
                    }
                } 
            }

            if is_grid_valid(&boards[k].1) != -1 {
                boards[k].0 = true;
            }
        }

        if only_1_remaining && boards[0].0 {
            return number * sum_of_grid(&boards[0].1);
        }

        boards.retain(|(has_won, _)| !has_won);

        if boards.len() == 1 {
            only_1_remaining = true;
        }
    }

    0
}
