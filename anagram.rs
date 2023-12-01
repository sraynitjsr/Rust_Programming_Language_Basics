use std::collections::HashMap;

fn are_anagrams(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut char_count = HashMap::new();

    for ch in str1.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    for ch in str2.chars() {
        if let Some(count) = char_count.get_mut(&ch) {
            *count -= 1;
            if *count == 0 {
                char_count.remove(&ch);
            }
        } else {            
            return false;
        }
    }

    char_count.is_empty()
}

fn main() {
    let word1 = "listen";
    let word2 = "silent";

    if are_anagrams(word1, word2) {
        println!("{} and {} are anagrams.", word1, word2);
    } else {
        println!("{} and {} are not anagrams.", word1, word2);
    }
}
