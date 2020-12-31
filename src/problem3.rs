pub fn run(n: i64) {
    let mut largest = 1;
    for i in 2..n+1 {
        if isPrime(i) && n % i == 0 {
            largest = i;
        }
    }
    println!("{}", largest);
}

fn isPrime(n: i64) -> bool {
    for x in 2..n {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}
