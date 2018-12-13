extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::pow::pow;
use num_traits::{Zero, One};

/// sum of digits
pub fn problem_377(){
    let mut sum: BigInt = Zero::zero();
    for i in 1..17 {
        let x = BigInt::from(i);
        let num = pow(x, 13);
        sum += sum(positive_integers(num));
    }
    println!("{}", sum);
}
/// all positive integers that do not have a zero
/// and digital sum equal to n
fn positive_integers(n: BigInt) -> Vec<&BigInt> {
    let mut x: BigInt = One::one();
    let mut vec: Vec<&BigInt> = vec!();

    while x <= get_max(&n) {
        if !(contains_zero(&x)) && digital_sum(&x) == n {
            vec.push(&x);
        }
    }

    return vec;
}

fn digital_sum(n: &BigInt) -> BigInt {
    let mut sum: BigInt = Zero::zero();

    let mut x = n.clone();
    while x > Zero::zero() {
        sum += BigInt::from(&x % 10);
        x /= 10;
    }
    return x;
}

fn sum(vec: Vec<&BigInt>) -> BigInt {
    let mut x = Zero::zero();

    for i in vec {
        x += i;
    }

    return x;
}

fn get_max(n: &BigInt) -> BigInt {
    let mut x: BigInt = n.clone();
    let mut num: String = String::new();

    while x != 0 {
        num.push_str("1");
        x -= One::one();
    }
    match BigInt::from_str(&num) {
        Ok(n) => n,
        Err(E) => panic!("{}", E),
    }
}

fn contains_zero(n: &BigInt) -> bool {
    return n.to_str_radix(10).contains("0");
}