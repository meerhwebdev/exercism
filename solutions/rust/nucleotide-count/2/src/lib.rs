use std::collections::HashMap;

fn check_valid_nucleotide(nucleotide:&char)-> bool{
    let list = ['A', 'T', 'G', 'C'];
    list.contains(nucleotide)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !check_valid_nucleotide(&nucleotide) {return Err('X')}
    let mut nucleotide_count:HashMap<char, usize> = HashMap::new();
    for ch in dna.chars() {
        if !check_valid_nucleotide(&ch) {return Err('X')}
        *nucleotide_count.entry(ch).or_default() += 1
    }
    match nucleotide_count.get(&nucleotide) {
        Some(&x)=> Ok(x),
        None => Ok(0)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotide_count:HashMap<char, usize> = HashMap::from([('A', 0),('T', 0),('G', 0),('C', 0)]);
    for ch in dna.chars() {
        if !check_valid_nucleotide(&ch) {return Err('X')}
        *nucleotide_count.entry(ch).or_default() += 1
    }
    Ok(nucleotide_count)
}