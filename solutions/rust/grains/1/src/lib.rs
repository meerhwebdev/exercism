pub fn square(s: u32) -> u64 {
    let mut grains = 1u64;
    if s <=0 || s > 64 {panic!("A standard chessboard has 64 individual squares")}
    for sq in 0..s {
        if sq == 0 {continue;}
        grains = grains * 2
    }
    grains
}


pub fn total() -> u64 {
    let mut total_grains = 1u64;
    let mut grains_in_prev = 1u64;
    for sq in 0..64 {
        if sq == 0 {continue;}
        grains_in_prev = grains_in_prev * 2;
        total_grains += grains_in_prev
    }
    total_grains
}