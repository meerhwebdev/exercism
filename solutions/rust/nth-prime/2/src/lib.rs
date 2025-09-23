pub fn nth(n: u32) -> u32 {
    let estimate = if n < 6 {
        15
    } else {
        let n_f = n as f64;
        (n_f * (n_f.ln() + n_f.ln().ln())).ceil() as usize
    };

    let mut sieve = vec![true; estimate + 1];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=((estimate as f64).sqrt() as usize) {
        if sieve[i] {
            for j in (i*i..=estimate).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    let primes: Vec<usize> = sieve.iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| i)
        .collect();

    primes[n as usize] as u32
}