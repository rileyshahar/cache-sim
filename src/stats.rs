//! Compute statistics for caches.

use std::collections::HashSet;

use super::Item;

pub trait Stat : std::fmt::Debug {
    fn update(&mut self, set: &HashSet<Item>, next: Item, evicted: Option<Item>);
}

#[derive(Default, Debug)]
pub struct HitRate {
    hits: u32,
    misses: u32,
}

impl Stat for HitRate {
    fn update(&mut self, set: &HashSet<Item>, next: Item, _: Option<Item>) {
        if set.contains(&next) {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
    }
}
