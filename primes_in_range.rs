use std::io;

fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }

    for i in 2..(num / 2 + 1) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Enter the lower bound of the range:");
    let mut lower_bound = String::new();
    io::stdin().read_line(&mut lower_bound).expect("Failed to read line");
    let lower_bound: u32 = lower_bound.trim().parse().expect("Please enter a valid number");

    println!("Enter the upper bound of the range:");
    let mut upper_bound = String::new();
    io::stdin().read_line(&mut upper_bound).expect("Failed to read line");
    let upper_bound: u32 = upper_bound.trim().parse().expect("Please enter a valid number");

    println!("Prime numbers in the range {} to {}:", lower_bound, upper_bound);

    for num in lower_bound..=upper_bound {
        if is_prime(num) {
            println!("{}", num);
        }
    }
}
