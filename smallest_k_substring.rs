fn smallest_k_substring(s: &str, k: usize) -> String {
    let mut result = String::new();
    
    let s_cyclic = {
        let mut cyclic = s.repeat(2);
        cyclic.truncate(s.len());
        cyclic
    };
    
    for _ in 0..k {
        result.push_str(&s_cyclic);
    }
    
    result
}

fn main() {
    let s = "abc";
    let k = 3;
    let smallest = smallest_k_substring(s, k);
    println!("{}", smallest);
}
