pub fn translate(input: &str) -> String {
    let mut translated = String::new();
    for word in input.split(' ') {
        let mut new_word:Vec<char> = word.chars().collect();
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        if word.starts_with("xr") || word.starts_with("yt") || word.starts_with(vowels) {
            new_word.extend(['a', 'y']);
        }else {
            let mut i = 0;
            let mut prev_ch:String = String::new();
            while i < new_word.len() {
                prev_ch.push(new_word[0]);
                if (vowels.contains(&new_word[0]) && prev_ch != "qu") || (prev_ch.ends_with("y") && prev_ch.len() > 1) {
                    new_word.extend(['a', 'y']);
                    break;
                }
                prev_ch = new_word[0].to_string();
                new_word.push(new_word[0]);
                new_word.remove(0);
                i += 1;
                
            }
        }
        match translated.len() {
            0 => translated.extend(new_word.iter()),
            _ => {
                translated.push(' ');
                translated.extend(new_word.iter())
            }
        }
    }
    translated
}