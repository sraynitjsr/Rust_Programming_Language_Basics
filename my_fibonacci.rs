fn my_fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut a = 0;
        let mut b = 1;
        let mut result = 0;

        for _ in 2..=n {
            result = a + b;
            a = b;
            b = result;
        }

        return result;
    }
}

fn main() {
    let n = 10;
    print!("First {} Fibonacci Numbers Are => ", n);
    print!("{}", my_fibonacci(0));
    for i in 1..=n-1 {
        print!(", {}", my_fibonacci(i));
    }
}
