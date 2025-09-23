pub fn is_armstrong_number(num: u32) -> bool {
    let mut number = 0u32;
    let num_str = num.to_string();
    let power  = num_str.len() as u32;
    for digit_char in num_str.chars() {
        let digit = digit_char.to_digit(10).unwrap();
        number += digit.pow(power);
    }
    if number == num {
        true
    } else {
        false
    }
}