#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 1 {return Some(Classification::Deficient)}
    if num == 0 {return None}
    let n = (num as f64).sqrt() as u64;
    let mut sum= 1u64;
    for i in 2..=n {
        if num % i == 0 {
            sum += i;
            if num / i != i {
                sum += num / i
            }
        }
    }
    match num as i64 - sum as i64 {
        0 => Some(Classification::Perfect),
        x if x < 0 => Some(Classification::Abundant),
        x if x > 0 => Some(Classification::Deficient),
        _ => None
    }
}