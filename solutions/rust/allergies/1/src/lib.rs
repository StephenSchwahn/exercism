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

fn calc_allergens(score: u32) -> Vec<Allergen> {
    let all_allergens = [
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];

    all_allergens
        .into_iter()
        .enumerate()
        .map(|(i, allergen)| { (allergen, 2u32.pow(i as u32)) })
        .filter(|(_, allergen_score)| (score & allergen_score != 0))
        .map(|(allergen, _)| allergen)
        .collect()
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            allergens: calc_allergens(score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
