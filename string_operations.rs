fn main() {
    let mut my_string = String::from("Hello, world!");
    my_string.push_str(" This is Rust.");
    println!("Original String: {}", my_string);

    let replaced_string = my_string.replace("Rust", "Awesome Rust");
    println!("Replaced String: {}", replaced_string);

    let uppercase_string = replaced_string.to_uppercase();
    println!("Uppercase String: {}", uppercase_string);

    let lowercase_string = replaced_string.to_lowercase();
    println!("Lowercase String: {}", lowercase_string);

    let split_string: Vec<&str> = replaced_string.split_whitespace().collect();
    println!("Split String: {:?}", split_string);

    let joined_string = split_string.join("-");
    println!("Joined String: {}", joined_string);

    if replaced_string.contains("world") {
        println!("The string contains 'world'.");
    } else {
        println!("The string does not contain 'world'.");
    }

    let trimmed_string = replaced_string.trim();
    println!("Trimmed String: {}", trimmed_string);

    if trimmed_string.starts_with("Hello") {
        println!("The string starts with 'Hello'.");
    } else {
        println!("The string does not start with 'Hello'.");
    }

    if trimmed_string.ends_with("Rust.") {
        println!("The string ends with 'Rust.'.");
    } else {
        println!("The string does not end with 'Rust.'.");
    }
}
