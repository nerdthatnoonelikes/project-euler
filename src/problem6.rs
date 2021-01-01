pub fn run() {
    let mut sum_squares = 0;
    let mut square_sum = 0;

    for i in 1..101 {
        sum_squares += i*i;
        square_sum += i;
    }
    square_sum = square_sum * square_sum;

    println!("{}", square_sum - sum_squares);
}
