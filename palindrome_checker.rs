fn is_palindrome_num(num: i32) -> bool {
    let num_str = num.to_string();
    let reversed_str: String = num_str.chars().rev().collect();
    num_str == reversed_str
}

fn is_palindrome_str(input: &str) -> bool {
    let input = input.to_lowercase();
    let reversed_str: String = input.chars().rev().collect();
    input == reversed_str
}

fn main() {
    let num = 12321;
    if is_palindrome_num(num) {
        println!("{} is a palindrome as a number.", num);
    } else {
        println!("{} is not a palindrome as a number.", num);
    }

    let word = "Deified";
    if is_palindrome_str(word) {
        println!("{} is a palindrome as a string.", word);
    } else {
        println!("{} is not a palindrome as a string.", word);
    }
}
