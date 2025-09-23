#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Superlist
    } else if is_sublist(second_list, first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }

}

fn is_sublist(first_list:&[i32], second_list:&[i32])-> bool {
    let f_str = first_list.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join("0,") + "0"; 
    let s_str = second_list.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join("0,") + "0";
    f_str.contains(&s_str)
}