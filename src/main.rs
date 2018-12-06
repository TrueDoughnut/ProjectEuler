#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::f64;

mod even_fibonacci;
mod palindrome_product;

fn main() {
    //problem_1();
    //even_fibonacci::run();
    //largest_prime_factor(600851475143);
    //palindrome_product::run();
    problem_5(20);
}


///multiples of 3 or 5 below 1000
fn problem_1(){
    let mut sum: i64 = 0;

    //multiples of 3
    for x in (3..1000).step_by(3) {
        sum += x;
    }

    //multiples of 5
    for x in (5..1000).step_by(5) {
        sum += x;
    }

    //subtract multiples of 3 and 5
    for x in (15..1000).step_by(15) {
        sum -= x;
    }

    println!("{}", sum);
}

///smallest evenly divisible multiple of first n numbers
fn problem_5(n: i32){
    let mut vec: Vec<i32> = vec!();
    for i in 1..n {
        vec.push(i);
    }

    let mut i = 1;

    while true {
        for i in vec {

        }

        i += 1;
    }
}

fn largest_prime_factor(x: i64) {
    let upper = (x as f64).sqrt() as i64 + 1;

    let vec = first_primes(1000);

    let mut max: i64 = 0;
    for i in 2..upper {
        if vec.contains(&i) && (x % i == 0) {
            max = i;
        }
    }

    println!("{}", max);
}

fn first_primes(n: i64) -> Vec<i64> {
    let mut vec: Vec<i64> = vec!((2));

    let mut i: i64 = 3;
    while vec.len() < n as usize {
        if is_prime(i) {
            vec.push(i);
        }
        i += 1;
    }

    println!("{:?}", vec);
    return vec;
}

fn is_prime(n: i64) -> bool {
    for i in 2..((n as f64).sqrt() as i64 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}