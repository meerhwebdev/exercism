#[allow(dead_code)]
pub fn encode(source: &str) -> String {
    let mut encoded_str = String::new();
    if source.is_empty() {return encoded_str}
    let chars_list:Vec<char> = source.chars().collect();
    let list_len = chars_list.len();
    let mut prev_ch:char = chars_list[0];
    let mut count:u32 = 1;
    for i in 1..=list_len {
        let ch = if i < list_len {
            chars_list[i]
        }else {
            '0'
        };
        if prev_ch == ch {
            count += 1;
        }else {
            if count > 1 {
                let count_ch = count.to_string();
                encoded_str.push_str(&count_ch);
            }
            encoded_str.push(prev_ch);
            count = 1;
            prev_ch = ch;
        }
    }
    encoded_str
}

#[allow(dead_code)]
pub fn decode(source: &str) -> String {
    let mut decoded_str = String::new();
    let chars_list:Vec<char> = source.chars().collect();
    let mut i= 0usize;
    while i < chars_list.len() {
        let ch = chars_list[i];
        if ch.is_digit(10) {
            let mut count = String::new();
            get_letter_count(&mut i, &chars_list, &mut count);
            let mut repeated_str = chars_list[i].to_string();
            repeated_str = repeated_str.repeat(count.parse::<usize>().unwrap() as usize);
            decoded_str.push_str(&repeated_str);
        }else {
            decoded_str.push(ch);
        }
        i += 1
    }
    decoded_str
}

fn get_letter_count(index:&mut usize, list:&Vec<char>, count:&mut String) {
    if let Some(_ch) =  list[*index].to_digit(10) {
        count.push(list[*index]);
        *index += 1;
        get_letter_count(index, list, count);
    } 
}