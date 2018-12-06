
pub fn run() {
    let mut max: i64 = 0;
    for x in 100..999 {
        for y in 100..999 {
            let product: i64  = x * y;
            if is_palindrome(product) && product > max {
                max = product;
            }
        }
    }
    println!("{}", max);
}

fn is_palindrome(n: i64) -> bool {
    let string = n.to_string();
    let reverse = string.chars().rev().collect::<String>();

    reverse.eq(&string)
}