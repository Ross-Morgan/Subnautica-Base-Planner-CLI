use std::collections::HashMap;

use super::{materials::Item, integrity::Integrity};


#[derive(Clone, Debug, Default)]
pub struct Base {
    pub items: HashMap<Item, usize>,
    pub depth: u32,
}


impl Base {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_item(&mut self, item: Item, quantity: usize) {
        self.items.entry(item).and_modify(|c| *c += quantity).or_insert(quantity);
    }

    pub fn remove_item(&mut self, item: Item, quantity: usize) {
        self.items.entry(item).and_modify(|c| {
            if quantity > *c { *c = 0; }
            else { *c -= quantity; }
        }).or_insert(0);
    }

    pub fn set_depth(&mut self, depth: i64) {
        self.depth = match depth {
            ..=0 => 0,
            d if d < std::u32::MAX.into() => d as u32,
            _ => { println!("Base depth too deep"); return }
        };
    }

    pub fn number_of(&self, item: &Item) -> usize {
        *self.items.get(item).unwrap_or(&0)
    }

    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    pub fn get_integrity(&self) -> f64 {
        Integrity::integrity(self)
    }
}

impl Integrity for Base {
    fn integrity(&self) -> f64 {
        let negative_integrity = self.items
            .iter()
            .filter_map(|(item, count)| {
                match item.integrity() {
                    i if i < 0.0 => Some(i * (*count as f64)),
                    _ => None
                }
            })
            .sum::<f64>();

        let positive_integrity = self.items
            .iter()
            .filter_map(|(item, count)| {
                match item.integrity() {
                    i if i > 0.0 => Some(i * (*count as f64)),
                    _ => None
                }
            })
            .sum::<f64>();

        let multiplier = match self.depth {
            0 => 0.0,
            1..=99 => 1.0,
            depth => {
                ((depth - 100) / 1000) as f64 + 1.0
            }
        };

        let base_integrity = ((negative_integrity  + 10.0)* multiplier) + positive_integrity;

        println!("Base integrity: {base_integrity}");
        println!("Multiplier: {multiplier}");

        base_integrity
     }
}
