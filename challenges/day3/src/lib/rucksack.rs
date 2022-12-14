use crate::container::Container;
use crate::item::Item;

pub struct Rucksack {
    containers: (Container, Container),
}

impl Rucksack {
    pub fn parse(rucksack_str: &str) -> Self {
        let full = rucksack_str.len();
        let half = full / 2;

        let mut rucksack = Rucksack{ containers: (Container::new(), Container::new())};
        
        let mut i = 0;
        for c in rucksack_str.chars() {
            if i < half {
                rucksack.containers.0.push(&c);
            } else {
                rucksack.containers.1.push(&c);
            }

            i+=1;
        }

        rucksack
    }

    pub fn shared_items(&self) -> Vec<Item> {
        let c1 = &self.containers.0;
        let c2 = &self.containers.1;

        c1.shared_items(c2)
    }
}