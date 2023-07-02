fn my_factorial(num:i64)->i64 {
	if num == 1 || num == 0 {
		return 1;
	} else {
		return num * my_factorial(num-1);
	}
}

fn main() {
    let num = 8;
    let res = my_factorial(num);
    println!("The factorial of {} is => {}",num, res);
}
