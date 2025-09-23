#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase)
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase)
    }
    let mut value = 0_u128;
    for &d in number {
        if d >= from_base {
            return Err(Error::InvalidDigit(d))
        }
        value = value * from_base as u128 + d as u128;
    }
    if value == 0 {
        return Ok(vec![0])
    }
    let mut output:Vec<u32> = Vec::new();
    let mut n  = value;
    while n > 0 {
        output.push((n % to_base as u128) as u32);
        n /= to_base as u128;
    }
    output.reverse();
    Ok(output)
}