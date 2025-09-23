pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        panic!("Please provide the substr length.")
    }
    let mut list:Vec<&str> = vec![];
    let digits_len = digits.len();
    let mut start_point:usize = 0;
    while start_point + len <= digits_len {
        list.push(&digits[start_point..start_point + len]);
        start_point += 1;
    }

    list.iter().map(|str| str.to_string()).collect()
}