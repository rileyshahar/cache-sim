//! Implementations of statistics computed by the cache simulator.

use std::collections::HashSet;

use crate::item::Item;

/// An abstract representation of a cache statistic.
///
/// Importantly, this is implemented for any tuple of statistics, up to size 12 (for technical
/// reasons we can't go higher than this right now). So to track multiple statistics, you can use a
/// tuple of statistics.
#[impl_trait_for_tuples::impl_for_tuples(12)] // can't go higher bc the stdlib doesn't impl default
                                              // for bigger tuples
pub trait Stat<I: Item>: Default {
    fn update(&mut self, set: &HashSet<I>, next: I, evicted: Option<I>);
}

/// The raw count of cache hits.
///
/// ```
/// use cache_sim::Cache;
/// use cache_sim::Lru;
/// use cache_sim::stats::HitCount;
///
/// let mut c = Cache::<Lru, HitCount>::new(3);
/// c.access(0); // miss
/// c.access(1); // miss
/// c.access(2); // miss
/// c.access(0); // hit
/// c.access(3); // miss
/// c.access(0); // hit
///
/// assert_eq!(c.stat().0, 2);
/// ```
#[derive(Default, Debug)]
pub struct HitCount(pub u32);

impl<I: Item> Stat<I> for HitCount {
    fn update(&mut self, set: &HashSet<I>, next: I, _: Option<I>) {
        if set.contains(&next) {
            self.0 += 1;
        }
    }
}

/// The raw count of cache misses.
///
/// ```
/// use cache_sim::Cache;
/// use cache_sim::Lru;
/// use cache_sim::stats::MissCount;
///
/// let mut c = Cache::<Lru, MissCount>::new(3);
/// c.access(0); // miss
/// c.access(1); // miss
/// c.access(2); // miss
/// c.access(0); // hit
/// c.access(3); // miss
/// c.access(0); // hit
///
/// assert_eq!(c.stat().0, 4);
/// ```
#[derive(Default, Debug)]
pub struct MissCount(pub u32);

impl<I: Item> Stat<I> for MissCount {
    fn update(&mut self, set: &HashSet<I>, next: I, _: Option<I>) {
        if !set.contains(&next) {
            self.0 += 1;
        }
    }
}
