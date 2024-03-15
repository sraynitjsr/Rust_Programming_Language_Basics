fn is_palindrome(num: i32) -> bool {
    let num_str = num.to_string();
    let reversed_str: String = num_str.chars().rev().collect();
    num_str == reversed_str
}

fn main() {
    let num = 12321;
    if is_palindrome(num) {
        println!("{} is a palindrome.", num);
    } else {
        println!("{} is not a palindrome.", num);
    }
}
