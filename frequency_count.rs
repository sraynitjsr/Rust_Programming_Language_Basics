use std::collections::HashMap;

fn main() {
    let array = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 1, 2, 1];
    let mut frequency_map: HashMap<i32, usize> = HashMap::new();

    for &element in &array {
        let count = frequency_map.entry(element).or_insert(0);
        *count += 1;
    }

    for (element, count) in &frequency_map {
        println!("Element {}: Count {}", element, count);
    }
}
