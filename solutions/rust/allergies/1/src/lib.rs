#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score & 0xFF }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let mask = Self::allergen_score(allergen);
        (self.score & mask) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let all = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        all.iter()
            .cloned()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }

    fn allergen_score(allergen: &Allergen) -> u32 {
        match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }
}