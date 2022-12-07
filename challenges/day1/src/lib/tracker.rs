use super::elf::Elf;
use super::strutil::is_numeric_string;

pub struct ElfTracker {
  elves: Vec<Elf>,
  most_calories: usize,
}

impl ElfTracker {
  pub fn new() -> ElfTracker {
      ElfTracker { elves: vec![], most_calories: 0 }
  }

  pub fn from(lines: Vec<String>) -> ElfTracker {
      let mut tracker = ElfTracker::new();
      let mut has_started_processing_elf = false;
      let mut current_elf: Elf = Elf::new();

      for line in lines.iter() {
          match line {
              i if i == &"" => {
                  if has_started_processing_elf {
                      tracker.add(current_elf);
                      current_elf = Elf::new();
                      has_started_processing_elf = false
                  }
              },
              i if is_numeric_string(&i) => {
                  let calorie_count = u64::from_str_radix(line, 10).unwrap_or_else(|_v| 0);
                  current_elf.add_food(calorie_count);
                  has_started_processing_elf = true;
              },
              _ => {
                  // no action to take, invalid line
              }
          }
      }

      if has_started_processing_elf {
          tracker.add(current_elf);
      }

      tracker
  }

  pub fn add(&mut self, elf: Elf) {
      if self.elves.len() > 0 && self.elves[self.most_calories].total < elf.total {
          self.most_calories = self.elves.len();
      }
      
      self.elves.push(elf);
  }

  pub fn len(&self) -> usize {
      self.elves.len()
  }

  pub fn get_elf_with_most_calories(&self) -> Elf {
      self.elves[self.most_calories].clone()
  }
}
