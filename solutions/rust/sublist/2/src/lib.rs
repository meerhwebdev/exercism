#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let f_str = first_list.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(","); 
    let s_str = second_list.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(",");
    if first_list == second_list {
        Comparison::Equal
    } else {
        if first_list.len() >= second_list.len() {
            if f_str.contains(&s_str) {
                Comparison::Superlist
            }else {
                Comparison::Unequal
            }
        } else {
            if s_str.contains(&f_str) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
    }
}