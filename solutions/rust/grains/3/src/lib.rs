pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("A standard chessboard has 64 individual squares");
    }
    1u64 << (s - 1) 
}

pub fn total() -> u64 {
    u64::MAX
}