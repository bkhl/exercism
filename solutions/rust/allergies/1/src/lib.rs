#[derive(Debug, Copy, Clone, PartialEq)]
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
        let value = 2_usize.pow(ALLERGENS.iter().position(|a| a == allergen).unwrap() as u32);
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
