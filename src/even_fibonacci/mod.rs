pub fn run(){
    let mut i = 1;
    let mut x = fib(i);
    let mut sum = 0;
    while x < 4000000 {

        if x % 2 == 0 {
            sum += x;
        }

        i += 1;
        x = fib(i);
    }
    println!("{}", sum);
}

fn fib(x: i64) -> i64 {
    if x == 1 {
        return 1;
    } else if x == 2{
        return 2;
    } else {
        return fib(x-1) + fib(x-2);
    }
}