use std::collections::HashMap;

fn main() {
    let input_string = "This is a Simple Sentence";

    let mut char_frequency: HashMap<char, usize> = HashMap::new();

    for c in input_string.chars() {
        if !c.is_whitespace() {
            let lowercase_char = c.to_lowercase().next().unwrap();

            *char_frequency.entry(lowercase_char).or_insert(0) += 1;
        }
    }

    println!("Character Frequency");
    for (ch, freq) in char_frequency {
        println!("Frequency of {} is => {}", ch, freq);
    }
}
