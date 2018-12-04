pub fn run(){
    let i = 1;


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