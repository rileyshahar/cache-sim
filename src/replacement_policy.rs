//! Constains replacement policies.

use super::Item;
use std::collections::{HashMap, HashSet};

use rand::seq::IteratorRandom;

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

/// The RAND replacement policy.
#[derive(Default)]
pub struct Rand;

impl ReplacementPolicy for Rand {
    fn update_state(&mut self, _: Item) {}

    fn replace(&mut self, set: &HashSet<Item>, _: usize, _: Item) -> Item {
        *set.iter()
            .choose(&mut rand::thread_rng())
            .expect("The set is non-empty.")
    }
}

/// The MRU replacement policy.
#[derive(Default)]
pub struct Mru {
    stack: Vec<Item>,
}

impl ReplacementPolicy for Mru {
    fn update_state(&mut self, next: Item) {
        if let Some(index) = self.stack.iter().position(|&i| i == next) {
            self.stack.remove(index);
        }

        self.stack.push(next);
    }

    fn replace(&mut self, _: &HashSet<Item>, _: usize, next: Item) -> Item {
        self.update_state(next);
        self.stack.pop().expect("The cache is non-empty.")
    }
}

/// The LFU replacement policy.
#[derive(Default)]
pub struct Lfu {
    counts: HashMap<Item, u32>,
}

impl ReplacementPolicy for Lfu {
    fn update_state(&mut self, next: Item) {
        *self.counts.entry(next).or_insert(0) += 1;
    }

    fn replace(&mut self, _: &HashSet<Item>, _: usize, next: Item) -> Item {
        self.update_state(next);
        *self
            .counts
            .iter()
            .min_by_key(|&(_, &count)| count)
            .expect("The frequency table is non-empty.")
            .0
    }
}
