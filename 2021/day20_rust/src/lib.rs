use std::fs;

const FILE: &str = "input.txt";

// image enhancement algorithm, input image
fn read_input() -> (Vec<bool>, Vec<Vec<bool>>)
{
    let contents= fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<String> = contents.lines().into_iter().map(|line| line.trim().to_owned()).collect();

    let algo = input[0].chars().map(|c| match c { '.' => false, _ => true}).collect::<Vec<bool>>();
    
    let mut input_image: Vec<Vec<bool>> = vec![];

    for l in input[2..].into_iter() {
        input_image.push(l.chars().map(|c| match c { '.' => false, _ => true}).collect::<Vec<bool>>());
    }

    (algo, input_image)
}

fn int_from_bits(bits: &Vec<bool>) -> i64 
{
    let mut i: i64 = 0;

    for b in bits {
        i = 2 * i + match b {
            true => 1,
            _ => 0
        };
    }

    i
}

// TRICKY THING : 
// since images are infinite, the default color (1 or 0) of 
// "outside of the image" pixels should be known for each iteration
// if the first character of the algo is ., then now prob : 0  gives 0
// but if it is #, then the default background color alternates
// between 0 and 1 over iterations
fn process_image(input: &Vec<Vec<bool>>, algo: &Vec<bool>, default_background: bool) -> Vec<Vec<bool>> 
{
    let (width, height) = (input.len(), input[0].len());

    let mut output: Vec<Vec<bool>> = vec![vec![false;height+2];width +2];

    for i in 0..(width+2) {
        for j in 0..(height+2){
            let mut bits: Vec<bool> = vec![default_background;9];
            // let mut newi: i64 = i as i64;
            // let mut newj: i64 = j as i64;
            let mut index = 0;

            for ni in [i as i64 -2, i as i64-1, i as i64] {
                for nj in [j as i64-2, j as i64-1, j as i64] {
                    if ni >= 0 && nj >= 0 && ni < width as i64 && nj < height as i64 {
                        bits[index] = input[ni as usize][nj as usize];
                    }
                    
                    index += 1;
                }
            }

            output[i][j] = algo[int_from_bits(&bits) as usize];
        }
    }

    output
}

#[allow(dead_code)]
fn print_image(image: &Vec<Vec<bool>>)
{
    println!();
    for l in image.iter() {
        for b in l.iter() {
            print!("{}", match b {
                true => "#",
                _ => "."
            });
        }
        println!();
    }
    println!();
}


// applies process_image n times
fn iter_image(image: &mut Vec<Vec<bool>>, algo: &Vec<bool>, n: i64) -> Vec<Vec<bool>>
{
    let mut default_background: bool = false;
    let mut output = image.clone();

    for _ in 0..n {
        output = process_image(&output, algo, default_background);

        if algo[0] {
            default_background = !default_background;
        }
    }

    output
}

pub fn result_1() -> i64
{
    let (algo,mut image) = read_input();
    let output = iter_image(&mut image, &algo, 2);
    output.into_iter().flat_map(|el| el).filter(|el| *el).count() as i64
}


pub fn result_2() -> i64
{   
    let (algo,mut image) = read_input();
    let output = iter_image(&mut image, &algo, 50);
    output.into_iter().flat_map(|el| el).filter(|el| *el).count() as i64
}