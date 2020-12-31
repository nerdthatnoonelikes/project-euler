pub fn run() {
    let mut fibNums = vec![1, 2];

    while fibNums[fibNums.len() - 1] < 4000000 {
        fibNums.push(fibNums[fibNums.len() - 1] + fibNums[fibNums.len() - 2]);
    }

    fibNums.remove(fibNums.len() - 1);

    let mut sum = 0;

    for x in fibNums {
        if x % 2 == 0 {
            sum += x;
        }
    }

    println!("{}", sum);
}
