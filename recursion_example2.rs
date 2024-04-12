fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n = 10;
    println!("Fibonacci series up to {}:", n);
    for i in 0..n {
        print!("{} ", fibonacci(i));
    }
}
