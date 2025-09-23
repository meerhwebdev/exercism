fn find_camel_case(word:&str) -> String {
    let mut camel_case_letter = String::new();
    let ch_word:Vec<char> =  word.chars().collect();
    for (index, ch) in ch_word.iter().enumerate() {
        if let Some(next_ch) = ch_word.get(index+1) {
            if ch.is_uppercase() && index > 0 && !next_ch.is_uppercase() {
                camel_case_letter.push(*ch);
            }
        }
    }
    camel_case_letter
}


pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let word_list:Vec<&str> = phrase.split(&[' ', '-', '_'][..]).map(|word| word.trim()).collect();
    for word in word_list {
        if let Some(acr) = word.chars().next() {
            acronym.push(acr);
            acronym.push_str(&find_camel_case(word));
        }
    }
    acronym.to_uppercase()
}
