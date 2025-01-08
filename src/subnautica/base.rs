use std::collections::HashMap;

use super::{integrity::Integrity, materials::Item};

#[derive(Clone, Debug, Default)]
pub struct Base {
    pub items: HashMap<Item, usize>,
    pub depth: u32,
}

impl Base {
    #[must_use] pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, item: Item, quantity: usize) {
        self.items
            .entry(item)
            .and_modify(|c| *c += quantity)
            .or_insert(quantity);
    }

    pub fn remove_item(&mut self, item: Item, quantity: usize) {
        self.items
            .entry(item)
            .and_modify(|c| {
                if quantity > *c {
                    *c = 0;
                } else {
                    *c -= quantity;
                }
            })
            .or_insert(0);
    }

    pub fn set_depth(&mut self, depth: i64) {
        self.depth = depth.max(0).unsigned_abs().try_into().unwrap_or(u32::MAX);
    }

    #[must_use] pub fn number_of(&self, item: &Item) -> usize {
        self.items.get(item).copied().unwrap_or(0)
    }

    #[must_use] pub fn get_depth(&self) -> u32 {
        self.depth
    }

    #[must_use] pub fn get_integrity(&self) -> f64 {
        Integrity::integrity(self)
    }
}
