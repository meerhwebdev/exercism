pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_without_hyphens = isbn.replace("-", "").to_uppercase();
    let isbn_char_list:Vec<char> = isbn_without_hyphens.chars().collect();
    if !isbn_char_list.iter().enumerate().all(|(index, &ch)| ch.is_ascii_digit() || (index == 9 && ch == 'X')) || isbn_char_list.len() != 10 {
        return false
    }
    let mut acc= 0usize;
    let mut index= 10usize;
    for ch in isbn_char_list  {
        let num = if index == 1 && ch == 'X' {
            10
        }else {
            ch.to_digit(10).expect("Numerical value was expected") as usize
        };
        acc += index * num;
        index -= 1;
    }
    acc % 11 == 0
}