//! Constains replacement policies.

use super::Item;
use std::collections::HashSet;

pub trait ReplacementPolicy {
    /// Return the item to be evicted.
    fn replace(&mut self, set: &HashSet<Item>, capacity: usize, next: Item) -> Option<Item>;
}

/// The LRU replacement policy.
#[derive(Default)]
pub struct Lru {
    stack: Vec<Item>,
}

impl Lru {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ReplacementPolicy for Lru {
    fn replace(&mut self, set: &HashSet<Item>, capacity: usize, next: Item) -> Option<Item> {
        let to_evict = if set.len() >= capacity {
            Some(self.stack.remove(0))
        } else {
            None
        };

        // insert it to the front of the stack, since it's least recently used
        if let Some(index) = self.stack.iter().position(|&i| i == next) {
            self.stack.remove(index);
        }

        self.stack.push(next);

        to_evict
    }
}
