pub fn run() {
    let mut nums: Vec<i64> = Vec::new();
    for num in 100..999 {
        for num2 in 100..num {
            let i = num * num2;
            if is_palindrome(i.to_string()) {
                nums.push(num * num2);
            }     
         }
    }

    let mut largest = nums[0];
    for x in nums {
        if x > largest {
            largest = x;
        }
    }

    println!("largest palindrome is {}", largest);
}

fn is_palindrome(string: String) -> bool {
    let mut i = 0;
    while i < string.len() - 1{
        i+=1;
        if string.as_bytes()[i] != string.as_bytes()[string.len() - 1 - i] {
         return false;
        }
    }
    true
}
