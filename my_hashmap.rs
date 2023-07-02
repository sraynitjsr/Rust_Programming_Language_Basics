use std::collections::HashMap;

fn main() {
    let mut hashmap: HashMap<String, i32> = HashMap::new();

    hashmap.insert(String::from("One"), 1);
    hashmap.insert(String::from("Two"), 2);
    hashmap.insert(String::from("Three"), 3);

    println!("Value for 'One' => {:?}", hashmap.get("One"));
    println!("Value for 'Two' => {:?}", hashmap.get("Two"));
    println!("Value for 'Three' => {:?}", hashmap.get("Three"));

    for (key, value) in &hashmap {
        println!("Key => {}, Value => {}", key, value);
    }

    hashmap.insert(String::from("Two"), 2020);

    hashmap.remove("Three");

    println!("Contains key 'One' => {}", hashmap.contains_key("One"));
    println!("Contains key 'Three' => {}", hashmap.contains_key("Three"));

    println!("Is HashMap Empty Before Clearing => {}", hashmap.is_empty());

    hashmap.clear();

    println!("Is HashMap Empty After Clearing => {}", hashmap.is_empty());
}
