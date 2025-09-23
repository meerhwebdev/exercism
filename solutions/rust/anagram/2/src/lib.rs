use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagram_set:HashSet<&str> = HashSet::new();
    for possible_word in possible_anagrams {
        if word.to_lowercase().eq(&possible_word.to_lowercase()) {
            continue;
        }
        if word_freq_map(word) == word_freq_map(possible_word) {
            anagram_set.insert(possible_word);
        }else {
            continue;
        }
    }
    anagram_set
}

fn word_freq_map(word:&str)-> HashMap<char, usize> {
    let mut  word_map = HashMap::new();
    for char in word.to_lowercase().chars() {
        *word_map.entry(char).or_insert(0) += 1
    }
    word_map
}