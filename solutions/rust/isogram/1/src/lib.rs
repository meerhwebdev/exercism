use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    if candidate.is_empty() {return true}
    let mut ch_count:HashMap<char, usize> = HashMap::new();
    for ch in candidate.to_lowercase().chars() {
        if ch_count.contains_key(&ch) && ch != '-' && ch != ' ' {
            return false
        }
        *ch_count.entry(ch).or_default() += 1
    }
    true
}