use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    min:u64,
    max:u64,
    value:u64
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let mut factors:HashSet<(u64, u64)> = HashSet::new();
        let mut i = 1_u64;
        let n = self.value;
        while i * i <= n {
            if n % i == 0 {
                let j = n / i;
                if i >= self.min && j >= self.min && j <= self.max {
                    factors.insert((i, j));
                }
            }
            i += 1
        }
        factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_pal :Option<u64> = None;
    let mut max_pal :Option<u64> = None;
    for val_1 in min..=max {
        for val_2 in val_1..=max {
            let product = val_1 * val_2;
            if is_palindrome(product) {
                min_pal = Some(min_pal.map_or(product, |val| val.min(product)));
                max_pal = Some(max_pal.map_or(product, |val| val.max(product)));
            }
        }
    }
    match (min_pal, max_pal) {
        (Some(min_val), Some(max_val)) => Some((
            Palindrome {
                min,
                max,
                value:min_val
            },
            Palindrome {
                min, 
                max,
                value:max_val
            }
        )),
        _ => None

    }
}

fn is_palindrome(num:u64)-> bool {
    let s = num.to_string();
    s.chars().eq(s.chars().rev()) 
}