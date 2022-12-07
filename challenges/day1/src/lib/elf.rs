#[derive(Clone)]
pub struct Elf {
    food: Vec<u64>,
    pub total: u64,
}

impl Elf {
    pub fn new() -> Elf {
        Elf { food: vec![], total: 0 }
    }

    pub fn add_food(&mut self, calories: u64) {
        self.food.push(calories);
        self.total = self.total + calories;
    }
}