pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors:Vec<u64> = vec![];
    let mut num_rem = n;
    let mut i = 2u64;
    while num_rem != 1 {
        if num_rem % i == 0 {
            num_rem = num_rem / i;
            prime_factors.push(i);
        }else {
            i += 1;
            continue
        }
    }
    prime_factors
}