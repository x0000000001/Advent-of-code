use std::fs;

const FILE: &str = "input.txt";

fn simulate(initial_fishes:&Vec<i64>, days: i64) -> i64 {
      //counts fishes at stage x, x from 0 to 8
      let mut fishes: Vec<i64> = vec![0;9];

      for f in initial_fishes {
          fishes[*f as usize] += 1;
      }
  
      for _ in 0..days {
          let temp = fishes[0];
  
          for i in 0..8{
              fishes[i] = fishes[i+1];
          }
  
          fishes[6] += temp;
          fishes[8] = temp;
  
  
          // for i in 0..fishes.len() {
          //     for _ in 0..fishes[i]{
          //         print!("{i} ");
          //     }
          // }
          // println!();
      }
  
      // for f in fishes.iter() {
      //     print!("{f} ");
      // }
      // println!();
  
      fishes.iter().sum()
}


pub fn result_1() -> i64
{  
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let initial_fishes: Vec<i64> = input[0].split(',').map(|el| el.parse().unwrap()).collect();

    simulate(&initial_fishes, 80)
}


pub fn result_2() -> i64
{   
    let contents = fs::read_to_string(FILE)
    .expect("Something went wrong reading the file");

    let input:Vec<_> = contents.lines().map(|line| line.trim()).collect();

    let initial_fishes: Vec<i64> = input[0].split(',').map(|el| el.parse().unwrap()).collect();

    simulate(&initial_fishes, 256)
}