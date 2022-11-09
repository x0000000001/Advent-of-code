use std::{fs, vec};

const FILE: &str = "input.txt";

pub fn result_1() -> i32
{  
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let height_map:Vec<Vec<u8>> = input.iter().
        map(|line| line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect())
        .collect();
    
    let width = height_map.len();
    let height = height_map[0].len();
    let mut sum: i32 = 0;

    for i in 0..width{
        for j in 0..height{
            //print!("{} ",height_map[i][j]);
            if (i == 0 || height_map[i-1][j] > height_map[i][j]) && 
                (i == (width -1) || height_map[i+1][j] > height_map[i][j]) &&
                (j == 0 || height_map[i][j-1] > height_map[i][j]) && 
                (j == (height -1) || height_map[i][j+1] > height_map[i][j]) {
                    //println!("{} ", height_map[i][j]);
                    sum += (height_map[i][j] as i32) + 1;
                }
        }
        //println!();
    }

    

    sum
}

fn basin_size(min_location: (i32,i32), heightmap: &Vec<Vec<u8>>) -> i32 {

    let width: i32 = heightmap.len() as i32;
    let height: i32 = heightmap[0].len() as i32;

    let mut to_explore_cases: Vec<(i32,i32)> = vec![min_location;1];
    let mut explored_cases: Vec<(i32,i32)> = Vec::new();

    while to_explore_cases.len() != 0 {
        let initial_length = to_explore_cases.len();
        for i in 0..initial_length {
            let (x,y) = to_explore_cases[i];
            if x != 0 && heightmap[(x-1) as usize][y as usize] != 9 && heightmap[(x-1) as usize][y as usize] > heightmap[x as usize][y as usize] && !explored_cases.contains(&(x-1, y)) && !to_explore_cases.contains(&(x-1, y)){
                to_explore_cases.push((x-1, y));
            }
            if y != 0 && heightmap[x as usize][(y-1) as usize] != 9 && heightmap[x as usize][(y-1) as usize] > heightmap[x as usize][y as usize] && !explored_cases.contains(&(x, y-1)) && !to_explore_cases.contains(&(x, y-1)){
                to_explore_cases.push((x, y-1));
            }
            if y != (height-1) && heightmap[x as usize][(y+1) as usize] != 9 && heightmap[x as usize][(y+1) as usize] > heightmap[x as usize][y as usize] && !explored_cases.contains(&(x, y+1)) && !to_explore_cases.contains(&(x, y+1)){
                to_explore_cases.push((x, y+1));
            }
            if x != width-1 && heightmap[(x+1) as usize][y as usize] != 9 && heightmap[(x+1) as usize][y as usize] > heightmap[x as usize][y as usize] && !explored_cases.contains(&(x+1, y)) && !to_explore_cases.contains(&(x+1, y)){
                to_explore_cases.push((x+1, y));
            }

            explored_cases.push((x,y));
        }

        to_explore_cases.drain(0..initial_length);
    }

    explored_cases.len() as i32
}

pub fn result_2() -> i32
{   
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let heightmap:Vec<Vec<u8>> = input.iter().
        map(|line| line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect())
        .collect();
    
    let width = heightmap.len();
    let height = heightmap[0].len();

    let mut min_locations: Vec<(i32,i32)> = Vec::new();

    for i in 0..width{
        for j in 0..height{
            //print!("{} ",height_map[i][j]);
            if (i == 0 || heightmap[i-1][j] > heightmap[i][j]) && 
                (i == (width -1) || heightmap[i+1][j] > heightmap[i][j]) &&
                (j == 0 || heightmap[i][j-1] > heightmap[i][j]) && 
                (j == (height -1) || heightmap[i][j+1] > heightmap[i][j]) {
                    //println!("{} ", height_map[i][j]);
                    min_locations.push((i as i32,j as i32));
                }
        }
        //println!();
    }

    let mut basins_sizes: Vec<i32> = min_locations.iter().map(|el| basin_size(*el, &heightmap)).collect();
    basins_sizes.sort();
    basins_sizes.reverse();

    // for b in basins_sizes.iter(){
    //     println!("{b}");
    // }

    return basins_sizes[0] * basins_sizes[1] * basins_sizes[2];
}