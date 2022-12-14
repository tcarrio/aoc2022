use std::collections::{HashMap};
use crate::item::Item;

pub struct Container {
    items: HashMap<char, Item>,
    total: u8,
}

impl Container {
    pub fn new() -> Self {
        Container { items: HashMap::new(), total: 0 }
    }

    pub fn push(&mut self, c: &char) {
        let item = Item::from(c);
        self.total += item.get_priority();
        self.items.insert(*c, item);
    }

    pub fn get_total(&self) -> u8 {
        self.total
    }

    pub fn shared_items<'a>(&self, other: &'a Container) -> Vec<Item> {
        self.items.iter()
                  .filter(|(k, _v)| -> bool { other.items.contains_key(k)})
                  .map(|(_k, v)| -> Item { v.clone() } )
                  .collect()
    }
}