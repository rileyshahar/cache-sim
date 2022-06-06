//! Compute statistics for caches.

use std::collections::HashSet;

use super::Item;

/// An abstract representation of a cache statistic.
///
/// Importantly, this is implemented for any tuple of statistics, up to size 12 (for technical
/// reasons we can't go higher than this right now). So to track multiple statistics, you can use a
/// tuple of statistics.
#[impl_trait_for_tuples::impl_for_tuples(12)] // can't go higher bc the stdlib doesn't impl default
                                              // for bigger tuples
pub trait Stat: Default {
    fn update(&mut self, set: &HashSet<Item>, next: Item, evicted: Option<Item>);
}

/// The raw count of cache hits.
///
/// ```
/// use cache_sim::{Cache, Item};
/// use cache_sim::replacement_policy::Lru;
/// use cache_sim::stats::HitCount;
///
/// let mut c = Cache::<Lru, HitCount>::new(3);
/// c.access(Item(0)); // miss
/// c.access(Item(1)); // miss
/// c.access(Item(2)); // miss
/// c.access(Item(0)); // hit
/// c.access(Item(3)); // miss
/// c.access(Item(0)); // hit
///
/// assert_eq!(c.stat().0, 2);
/// ```
#[derive(Default, Debug)]
pub struct HitCount(pub u32);

impl Stat for HitCount {
    fn update(&mut self, set: &HashSet<Item>, next: Item, _: Option<Item>) {
        if set.contains(&next) {
            self.0 += 1;
        }
    }
}

/// The raw count of cache misses.
///
/// ```
/// use cache_sim::{Cache, Item};
/// use cache_sim::replacement_policy::Lru;
/// use cache_sim::stats::MissCount;
///
/// let mut c = Cache::<Lru, MissCount>::new(3);
/// c.access(Item(0)); // miss
/// c.access(Item(1)); // miss
/// c.access(Item(2)); // miss
/// c.access(Item(0)); // hit
/// c.access(Item(3)); // miss
/// c.access(Item(0)); // hit
///
/// assert_eq!(c.stat().0, 4);
/// ```
#[derive(Default, Debug)]
pub struct MissCount(pub u32);

impl Stat for MissCount {
    fn update(&mut self, set: &HashSet<Item>, next: Item, _: Option<Item>) {
        if !set.contains(&next) {
            self.0 += 1;
        }
    }
}
