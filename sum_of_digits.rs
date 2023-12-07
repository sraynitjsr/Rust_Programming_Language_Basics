use std::io;

fn main() {    
    println!("Enter a number => ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: u32 = input.trim().parse().expect("Please enter a valid number");
    let sum = sum_of_digits(number);
    println!("Sum of digits => {}", sum);
}

fn sum_of_digits(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += digit;
        n /= 10;
    }
    sum
}
