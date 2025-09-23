pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {return None}
    let mut hamming_dist = 0;
    let s1_chars:Vec<char> = s1.chars().collect();
    let s2_chars:Vec<char> = s2.chars().collect();
    for index in 0..s1.len() {
        if s1_chars[index] != s2_chars[index] {
            hamming_dist += 1
        }
    }
    Some(hamming_dist)
}