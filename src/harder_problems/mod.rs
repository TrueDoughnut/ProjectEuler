extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::pow::pow;
use num_traits::{Zero, One};

/// sum of digits
fn problem_377(){

}
/// all positive integers that do not have a zero
/// and digital sum equal to n
fn positive_integers(n: BigInt) -> Vec<BigInt> {
    let mut x: BigInt = One::one();

    let max: BigInt;

    for x in
}