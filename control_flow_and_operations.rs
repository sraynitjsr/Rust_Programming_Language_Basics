fn main() {
    let number = 10;

    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    println!("Printing numbers from 1 to 5 using a for loop:");
    for i in 1..=5 {
        println!("{}", i);
    }

    println!("Printing numbers from 5 to 1 using a while loop:");
    let mut j = 5;
    while j > 0 {
        println!("{}", j);
        j -= 1;
    }

    let a = 5;
    let b = 3;

    println!("Arithmetic operations:");
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);
    println!("{} % {} = {}", a, b, a % b);

    let c = 12;
    let d = 4;

    println!("Additional arithmetic operations:");
    println!("{} + {} = {}", c, d, c + d);
    println!("{} - {} = {}", c, d, c - d);
    println!("{} * {} = {}", c, d, c * d);
    println!("{} / {} = {}", c, d, c / d);
    println!("{} % {} = {}", c, d, c % d);
    println!("{} raised to the power of {} = {}", c, d, c.pow(d));
    println!("Square root of {} = {}", c, (c as f64).sqrt());
}
