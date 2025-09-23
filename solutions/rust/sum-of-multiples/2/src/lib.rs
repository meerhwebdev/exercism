use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples:HashSet<u32> = HashSet::new();
    for num in factors {
        if num > &0_u32 {
            for i in (0..limit).step_by(*num as usize) {
            multiples.insert(i);
            }
        }
    }
    multiples.iter().sum::<u32>()
}