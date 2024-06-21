use std::collections::HashMap;

struct Cache {
    map: HashMap<String, u64>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            map: HashMap::new(),
        }
    }

    fn get_or_compute(&mut self, key: &str) -> u64 {
        if let Some(&value) = self.map.get(key) {
            return value;
        }

        let computed_value = self.expensive_computation(key);
        self.map.insert(key.to_string(), computed_value);
        computed_value
    }

    fn expensive_computation(&self, key: &str) -> u64 {
        key.len() as u64
    }
}

fn main() {
    let mut cache = Cache::new();

    println!("First call:");
    println!("Result: {}", cache.get_or_compute("hello"));

    println!("\nSecond call:");
    println!("Result: {}", cache.get_or_compute("world"));

    println!("\nThird call:");
    println!("Result: {}", cache.get_or_compute("hello"));

    println!("\nFourth call:");
    println!("Result: {}", cache.get_or_compute("Rust"));
}
