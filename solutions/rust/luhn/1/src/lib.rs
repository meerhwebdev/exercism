pub fn is_valid(code: &str) -> bool {
    let mut luhn_list:Vec<u32> = vec![];
    let clean_str = &code.replace(" ", "");
    if clean_str.chars().any(|ch| !ch.is_numeric()) || clean_str == "0" {
        return false;
    }
    let code_chars = clean_str.chars();
    for (i, ch) in code_chars.rev().enumerate() {
        let digit = ch.to_digit(10).unwrap();
        if (i+1) % 2 == 0 {
            let num = if digit * 2 > 9 {
                digit * 2 - 9
            }else {
                digit * 2
            };
            luhn_list.push(num);
        }else {
            luhn_list.push(digit);   
        }
    }
    luhn_list.iter().sum::<u32>() % 10 == 0
}