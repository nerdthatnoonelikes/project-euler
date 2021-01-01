pub fn run() {
    let mut num = 1;
    while true {
        if divisble(num) {
            break
        } else {
            num += 1;
        }
    }
    println!("{}", num);
}

fn divisble(n: i32) -> bool {
    for i in 1..21 {
        if n % i != 0 {
            return false;
        }
    }
    return true;
}
