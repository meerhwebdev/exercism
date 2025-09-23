pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    let list_len = list.len();
    if list_len == 0 { return proverb}
    let list_to_use = if list_len > 0 {
        list_len - 1
    } else {
        0
    };

    for i in 0..list_to_use {
        proverb.push_str(
            &format!(
                "For want of a {} the {} was lost.\n",
                list[i], list[i+1]
            )
        )

    }
    proverb.push_str(
        &format!(
            "And all for the want of a {}.", list[0]
        )
   );
    proverb
}