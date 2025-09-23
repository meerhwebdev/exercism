pub fn reverse(input: &str) -> String {
    let mut rev_str = "".to_string();
    let mut str_list:Vec<char> = input.chars().collect();
    str_list.reverse();
    for char in str_list  {
        let new_str = format!("{rev_str}{char}");
        rev_str = new_str
    }
    rev_str
}
