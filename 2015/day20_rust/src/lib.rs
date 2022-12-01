pub type InputType = u64;


pub fn result_1(_input: InputType) -> i64
{   
    let input = (_input/ 10) as usize;
    let mut sums = vec![0;input as usize];

    for i in 1..input {
        for j in 1..(input/i) {
            sums[i*j] += i;
        }
    }

    for i in 0..sums.len() {
        if sums[i] >= input {
            return i as i64;
        }
    }

    0
}

pub fn result_2(_input: InputType) -> i64
{   
    let input = (_input/ 11) as usize;
    let mut sums = vec![0;input as usize];

    for i in 1..input {
        for j in 1..(input/i).min(51) {
            sums[i*j] += i;
        }
    }

    for i in 0..sums.len() {
        if sums[i] >= input {
            return i as i64;
        }
    }

    0
}






























// haha forget all about this, being dumb is much faster :)
// (each number's gifts in 1st problem is the sum of its divisors)


// fn add_one_prime(primes: &mut Vec<u64>) {
//     let mut curr = primes.last().unwrap().clone()+1;
//     loop {
//         let sr = f64::sqrt(curr as f64).ceil() as u64;
//         let mut is_prime = true;
//         for i in 0..(primes.len()) {
//             if primes[i] > sr {
//                 break;
//             }
//             if curr%primes[i] == 0 {
//                 is_prime = false;
//                 break;
//             }
//         }

//         if is_prime {
//             primes.push(curr.clone());
//             break;
//         }

//         curr += 1;
//     }
// }

// fn prime_factorization(mut n: u64, primes: &mut Vec<u64>) -> Vec<u64> {
//     let mut f = vec![];

//     let limit = n/2+1;
//     let mut i = 0;

//     while n != 1 && primes[i] <= limit {
//         let p = primes[i];

//         while n%p == 0 {
//             n /= p;
//             f.push(p);
//         }

//         if n == 0 || p > limit{
//             break;
//         }

//         i += 1;

//         if i == primes.len() {
//             add_one_prime(primes);
//         }
//     }

//     f
// }

// fn sum_of_factors(n: u64, fac: &Vec<u64>) -> u64 {
//     if fac.is_empty() {
//         return n + 1;
//     }
//     let mut res = 1;
//     let mut temp = fac[0];
//     let mut count = 0;

//     for p in fac {
//         if *p == temp {
//             count += 1;
//         } else {
//             res *= (temp.pow(count+1 as u32) - 1)/(temp - 1);
//             temp = *p;
//             count = 1;
//         }
//     }

//     res *= (temp.pow(count+1 as u32) - 1)/(temp - 1);

//     res
// }

// /// a sum of divisors problem
// pub fn result_1(mut input: InputType) -> i64
// {
//     // useless 0
//     input /= 10;

//     let mut primes: Vec<u64> = vec![2;1];

//     let mut i = 1;
//     loop {
//         if i %10000 == 0 {
//             println!("{i}");
//         }

//         i += 1;

//         if sum_of_factors(i, &prime_factorization(i, &mut primes)) >= input {
//             return i as i64;
//         }
//     }
// }

// pub fn result_2(mut input: InputType) -> i64
// {   
//     0
// }