//! Constains replacement policies.

use super::Item;
use std::collections::HashSet;

pub trait ReplacementPolicy {
    /// Update the replacement policy's state, without evicting an item.
    fn update_state(&mut self, next: Item);

    /// Return the item to be evicted.
    fn replace(&mut self, set: &HashSet<Item>, capacity: usize, next: Item) -> Item;
}

/// The LRU replacement policy.
#[derive(Default)]
pub struct Lru {
    stack: Vec<Item>,
}

impl ReplacementPolicy for Lru {
    fn update_state(&mut self, next: Item) {
        if let Some(index) = self.stack.iter().position(|&i| i == next) {
            self.stack.remove(index);
        }

        self.stack.push(next);
    }

    fn replace(&mut self, _: &HashSet<Item>, _: usize, next: Item) -> Item {
        self.update_state(next);
        self.stack.remove(0)
    }
}

/// The FIFO replacement policy.
#[derive(Default)]
pub struct Fifo {
    stack: Vec<Item>,
}

impl ReplacementPolicy for Fifo {
    fn update_state(&mut self, next: Item) {
        if !self.stack.contains(&next) {
            self.stack.push(next);
        }
    }

    fn replace(&mut self, _: &HashSet<Item>, _: usize, next: Item) -> Item {
        self.update_state(next);
        self.stack.remove(0)
    }
}
