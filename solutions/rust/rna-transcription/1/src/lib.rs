use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    coding_strand:String
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    coding_strand:String
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides = ['A', 'T', 'C', 'G'];
        let mut seq = String::new();
        for (index, ch) in dna.to_uppercase().chars().enumerate() {
            if nucleotides.contains(&ch) {
                seq.push(ch);
            }else {
                return Err(index)
            }
        }
        Ok(Self { coding_strand: seq  })
    }

    pub fn into_rna(self) -> Rna {
        let pair = HashMap::from([('G','C'),('C','G'), ('T','A'), ('A','U')]);
        let mut trans_seq = String::new();
        for ch in self.coding_strand.chars() {
            trans_seq.push(pair.get(&ch).unwrap().to_owned())
        }
        Rna { coding_strand: trans_seq }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
         let nucleotides = ['A', 'C', 'G', 'U'];
        let mut seq = String::new();
        for (index, ch) in rna.to_uppercase().chars().enumerate() {
            if nucleotides.contains(&ch) {
                seq.push(ch);
            }else {
                return Err(index)
            }
        }
        Ok(Self { coding_strand: seq  })
    }
}
