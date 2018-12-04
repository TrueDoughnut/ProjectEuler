mod even_fibonacci;

fn main() {
    //problem_1();
    even_fibonacci::run();
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