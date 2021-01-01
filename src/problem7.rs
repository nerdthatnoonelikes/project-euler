// just watch the numbers print on screen and the last printed number is the 1001st prime
pub fn run() {
    let mut num = 2;
    let mut primes = 0;
    while primes != 10001 {
        if isPrime(num) {
            println!("{} is prime", num);
            primes += 1;
        }
        num += 1;
    }

    
}

fn isPrime(n: i64) -> bool {
    for x in 2..n {
        if n % x == 0 {
            return false;                    
        }
    }
    return true;    
}
