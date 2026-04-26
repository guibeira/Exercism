pub struct Allergies {
    score: u32,
}

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

const ALLERGENS: [(u32, Allergen); 8] = [
    (1, Allergen::Eggs),
    (2, Allergen::Peanuts),
    (4, Allergen::Shellfish),
    (8, Allergen::Strawberries),
    (16, Allergen::Tomatoes),
    (32, Allergen::Chocolate),
    (64, Allergen::Pollen),
    (128, Allergen::Cats),
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let list_allergen = self.allergies();
        list_allergen.contains(&allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter_map(|&(score, allergen)| (self.score & score != 0).then_some(allergen))
            .collect()
    }
}
