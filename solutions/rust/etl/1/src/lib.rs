use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output:BTreeMap<char, i32> = BTreeMap::new();
    for (&num, chars) in h {
        for &ch in chars {
            output.insert(ch.to_ascii_lowercase(), num);
        }
    }
    output
}
