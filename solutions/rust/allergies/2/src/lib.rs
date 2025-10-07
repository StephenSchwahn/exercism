pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 0,     // 2^0 == 1
    Peanuts,      // 2^1 == 2
    Shellfish,    // 4
    Strawberries, // 2^3 == 8
    Tomatoes,     // 16
    Chocolate,    // 32
    Pollen,       // 64
    Cats,         // 128
}
const ALL_ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            allergens: ALL_ALLERGENS
                .into_iter()
                .filter(|&allergen| (score & 2u32.pow(allergen as u32) != 0))
                .collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
