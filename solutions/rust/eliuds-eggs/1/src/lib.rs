pub fn egg_count(display_value: u32) -> usize {
    if display_value == 0 { return 0};
    let mut num = display_value;
    let mut num_eggs = 0_usize;

    while num > 0 {
        if 1 & num == 1 {
            num_eggs += 1
        }
        num >>= 1
    }
    num_eggs
}