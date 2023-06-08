use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your name => ");
    
    io::stdin().read_line(&mut input).expect("Failed to read input");

    println!("Hello, {}!", input);
}
