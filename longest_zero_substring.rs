fn longest_zero_substring(s: &str, k: usize) -> String {
    let mut max_substring = String::new();
    
    let concatenated = s.repeat(k);
    
    let mut current_substring = String::new();
    for c in concatenated.chars() {
        if c == '0' {
            current_substring.push('0');
        } else {
            if current_substring.len() > max_substring.len() {
                max_substring = current_substring.clone();
            }
            current_substring.clear();
        }
    }
    
    if current_substring.len() > max_substring.len() {
        max_substring = current_substring;
    }
    
    max_substring
}

fn main() {
    let binary_string = "101000110011100010";
    let k = 3;
    
    let longest_zero_substring = longest_zero_substring(binary_string, k);
    println!("{}", longest_zero_substring);
}
