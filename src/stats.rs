//! Compute statistics for caches.

use std::collections::HashSet;

use super::Item;

#[impl_trait_for_tuples::impl_for_tuples(12)] // can't go higher bc the stdlib doesn't impl default
                                              // for bigger tuples
pub trait Stat: Default {
    fn update(&mut self, set: &HashSet<Item>, next: Item, evicted: Option<Item>);
}

#[derive(Default, Debug)]
pub struct HitCount(u32);

impl Stat for HitCount {
    fn update(&mut self, set: &HashSet<Item>, next: Item, _: Option<Item>) {
        if set.contains(&next) {
            self.0 += 1;
        }
    }
}

#[derive(Default, Debug)]
pub struct MissCount(u32);

impl Stat for MissCount {
    fn update(&mut self, set: &HashSet<Item>, next: Item, _: Option<Item>) {
        if !set.contains(&next) {
            self.0 += 1;
        }
    }
}
