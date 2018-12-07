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
    //problem_5(20);
    //problem_6(100);
    //problem_7(10001);
    //problem_8();
    //problem_9();
    //problem_10();
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

/// smallest evenly divisible multiple of first n numbers
fn problem_5(n: i32){
    let mut vec: Vec<i32> = vec!();
    for i in 1..n {
        vec.push(i);
    }

    let mut x = n.clone();

    loop {
        let mut divisible = true;
        for i in vec.iter() {
            if x % i != 0 {
                divisible = false;
                break;
            }
        }
        if divisible {
            println!("{}", x);
            break;
        }

        if n >= 10 {
            if x % 10 != 0 {
                x += x % 10;
            } else {
                x += 10;
            }
        } else {
            x += 1;
        }
    }
}

/// difference of sum of squares and square of sum of first n natural numbers
fn problem_6(n: i64) {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i;
    }
    let square_of_sum = sum * sum;

    let mut sum_of_square = 0;
    for i in 1..n+1 {
        sum_of_square += i * i;
    }

    let ans = square_of_sum - sum_of_square;

    println!("{}", ans);
}

/// nth prime
fn problem_7(n: i32) {
    let vec = first_primes(n as i64);
    let last = match vec.last() {
        Some(num) => num,
        None => &-1,
    };
    println!("{}", last);
}

/// largest product in series
///
fn problem_8() {
    let num = String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");
    let max: i64 = 0;
    for i in 0..num.len() - 13 {
        let small = &num[i..i+13];
        let mut product: i64 = 1;
        for char in small.chars() {
            let x = match char.to_digit(10) {
                Some(num) => num,
                None => panic!(),
            };
            product *= x as i64;
        }

        if product > max {
            let max = product;
        }
    }

    println!("{}", max);
}

///special pythagorean triplet
fn problem_9() {
    let mut a = 1;
    let mut b = 1;
    let mut c = 1;

    'outer: while a < 1000 {
        while b < 1000 {
            while c < 1000 {
                if a * a + b * b == c * c && (a + b + c) == 1000 {
                    break 'outer;
                }
                c += 1;
            }
            b += 1;
            c = 1;
        }
        a += 1;
        b = 1;
        c = 1;
    }

    println!("{}", a * b * c);
}

///summation of primes
fn problem_10() {
    let vec = first_primes(250000);
    let mut i = 0;
    let mut sum = 0;
    while vec[i] <= 2000000 {
        sum += vec[i];
        i += 1;
    }
    println!("{}", sum);
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

fn num_divisors(n: i64) -> i64 {
    let mut count: i64 = 0;

    for i in 2..((n as f64).sqrt() as i64 + 1) {
        if n % i == 0 {
            count += 1;
        }
    }

    return count;
}