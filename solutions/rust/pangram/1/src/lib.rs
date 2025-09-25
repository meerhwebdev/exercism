 use std::collections::HashMap;

pub fn is_pangram(sentence: &str) -> bool {
    let mut char_count:HashMap<char, usize> = HashMap::new();
    for ch in sentence.to_lowercase().chars(){
        if ch.is_alphabetic() {
            *char_count.entry(ch).or_default() += 1; 
        }
    }
    char_count.len() == 26
}