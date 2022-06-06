//! Contains utilities for managing a cache.

use std::collections::HashSet;
use std::fmt::Display;

use super::replacement_policy::ReplacementPolicy;
use super::stats::Stat;
use super::Item;

/// A cache, generic over a replacement policy and set of statistics.
///
/// Basic usage:
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::{Cache, Item};
/// use cache_sim::replacement_policy::Lru;
///
/// let mut c = Cache::<Lru>::new(3);
///
/// c.access(Item(0));
/// c.access(Item(1));
/// c.access(Item(2));
/// c.access(Item(0));
/// c.access(Item(3));
///
/// assert_eq!(c.set(), &HashSet::from([Item(0), Item(2), Item(3)]));
/// ```
///
/// The cache tracks the statistics represented by the type S (default to none):
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
///
/// To track multiple statistics, use a tuple of statistics:
/// ```
/// use cache_sim::{Cache, Item};
/// use cache_sim::replacement_policy::Lru;
/// use cache_sim::stats::{HitCount, MissCount};
///
/// let mut c = Cache::<Lru, (HitCount, MissCount)>::new(3);
/// c.access(Item(0)); // miss
/// c.access(Item(1)); // miss
/// c.access(Item(2)); // miss
/// c.access(Item(0)); // hit
/// c.access(Item(3)); // miss
/// c.access(Item(0)); // hit
///
/// assert_eq!(c.stat().0.0, 2);
/// assert_eq!(c.stat().1.0, 4);
/// ```
///
pub struct Cache<R: ReplacementPolicy, S: Stat = ()> {
    set: HashSet<Item>,
    replacement_policy: R,
    capacity: usize,
    stat: S,
}

impl<R: ReplacementPolicy, S: Stat> Cache<R, S> {
    /// Create an empty cache using an explicitly configured replacement policy.
    pub fn with_replacement_policy(policy: R, capacity: usize) -> Self {
        Self {
            set: HashSet::default(),
            replacement_policy: policy,
            capacity,
            stat: S::default(),
        }
    }

    /// Update the cache after an access to item.
    pub fn access(&mut self, item: Item) {
        if self.set.len() < self.capacity || self.set.contains(&item) {
            // we're assuming demand caching for now, so here we don't need to change anything in
            // the cache, and we just update the state of the replacement policy and the statistics
            self.replacement_policy.update_state(item);
            self.stat.update(&self.set, item, None);
        } else {
            // here we actually need to evict something
            let to_evict = self
                .replacement_policy
                .replace(&self.set, self.capacity, item);

            // TODO: is there an easy restructuring of this that prevents us from evicting and then
            // reinserting `item`, thus ending with an over capacity cache? This can happen now if
            // the replacement policy is implemented incorrectly.
            self.stat.update(&self.set, item, Some(to_evict));
            self.set.remove(&to_evict);
        }

        // finally, again because we assume demand paging, we always have to put the last access
        // into the cache
        self.set.insert(item);
    }

    /// Get a reference to cache's statistic.
    pub const fn stat(&self) -> &S {
        &self.stat
    }

    /// Get a reference to cache's set of items.
    pub const fn set(&self) -> &HashSet<Item> {
        &self.set
    }
}

impl<R: ReplacementPolicy + Default, S: Stat> Cache<R, S> {
    /// Create an empty cache using the default parameters for the replacement policy.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            set: HashSet::default(),
            replacement_policy: R::default(),
            capacity,
            stat: S::default(),
        }
    }
}

// An implementation of printing a cache.
impl<R: ReplacementPolicy, S: Stat> Display for Cache<R, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.set.iter().max().map_or(0, |i| i.0) < 26 {
            for item in &self.set {
                // prints the letter of each item, i.e. 0 -> A, 1 -> B, etc
                write!(
                    f,
                    "{}",
                    char::from_u32(item.0 + 'A' as u32)
                        .expect("all elements of list are valid chars")
                )?;
            }
        } else {
            for (i, item) in self.set.iter().enumerate() {
                // prints the number associated with each item in the stack, in order
                if i == self.set.len() - 1 {
                    write!(f, "{}", item.0)?;
                } else {
                    write!(f, "{}, ", item.0)?;
                }
            }
        }

        Ok(())
    }
}
