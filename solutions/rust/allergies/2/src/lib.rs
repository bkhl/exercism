#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

static ALLERGENS: &'static [Allergen] = &[
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

pub struct Allergies {
    score: usize,
}

impl Allergies {
    pub fn new(score: usize) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let value = *allergen as usize;
        self.score & value == value
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result: Vec<Allergen> = vec![];
        let mut value = 1;

        for allergen in ALLERGENS.iter() {
            if self.score & value == value {
                result.push(*allergen);
            }
            value *= 2;
        }

        result
    }
}
