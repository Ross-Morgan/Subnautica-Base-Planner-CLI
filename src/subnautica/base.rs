use std::collections::HashMap;

use super::{materials::Item, integrity::Integrity};


#[derive(Clone, Debug, Default)]
pub struct Base {
    items: HashMap<Item, usize>,
    depth: u32,
}

impl Base {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_item(&mut self, item: Item, quantity: usize) {
        self.items.entry(item).and_modify(|c| *c += quantity).or_insert(quantity);
    }

    pub fn set_depth(&mut self, depth: i64) {
        self.depth = match depth {
            ..=0 => 0,
            d if d < std::u32::MAX.into() => d as u32,
            _ => { println!("Base depth too deep"); return }
        };
    }

    pub fn get_depth(&self) -> u32 {
        self.depth
    }
}

impl Integrity for Base {
    fn integrity(&self) -> f64 {
        let base_integrity = self.items
            .iter()
            .map(|(item, count)| {
                Integrity::integrity(item) * (*count as f64)
            })
            .sum::<f64>();

        match self.depth {
            0 => 0.0,
            1..=99 => base_integrity,
            depth => {
                base_integrity * ((depth - 100) / 1000) as f64 + 1.0
            }
        }
    }
}
