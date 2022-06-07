//! Implementations of cache replacement policies.

use crate::item::Item;
use std::collections::{HashMap, HashSet};

use rand::seq::IteratorRandom;

/// An abstracted cache replacement policy.
pub trait ReplacementPolicy<I: Item> {
    /// Update the replacement policy's state, without evicting an item.
    fn update_state(&mut self, next: I);

    /// Return the item to be evicted. This should _not_ be `next`.
    fn replace(&mut self, set: &HashSet<I>, capacity: usize, next: I) -> I;
}

/// The LRU replacement policy, which evicts the least recently used item.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::Cache;
/// use cache_sim::replacement_policy::Lru;
///
/// let mut c = Cache::<Lru>::new(3);
///
/// c.access(0);
/// c.access(1);
/// c.access(2);
/// c.access(0);
/// c.access(3);
///
/// assert_eq!(c.set(), &HashSet::from([0, 2, 3]));
/// ```
#[derive(Default)]
pub struct Lru<I: Item = u32> {
    stack: Vec<I>,
}

impl<I: Item> ReplacementPolicy<I> for Lru<I> {
    fn update_state(&mut self, next: I) {
        if let Some(index) = self.stack.iter().position(|&i| i == next) {
            self.stack.remove(index);
        }

        self.stack.push(next);
    }

    fn replace(&mut self, _: &HashSet<I>, _: usize, next: I) -> I {
        self.update_state(next);
        self.stack.remove(0)
    }
}

/// The FIFO replacement policy, which evicts the first-inserted item.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::Cache;
/// use cache_sim::replacement_policy::Fifo;
///
/// let mut c = Cache::<Fifo>::new(3);
///
/// c.access(0);
/// c.access(1);
/// c.access(2);
/// c.access(0);
/// c.access(3);
///
/// assert_eq!(c.set(), &HashSet::from([1, 2, 3]));
/// ```
#[derive(Default)]
pub struct Fifo<I: Item = u32> {
    stack: Vec<I>,
}

impl<I: Item> ReplacementPolicy<I> for Fifo<I> {
    fn update_state(&mut self, next: I) {
        if !self.stack.contains(&next) {
            self.stack.push(next);
        }
    }

    fn replace(&mut self, _: &HashSet<I>, _: usize, next: I) -> I {
        self.update_state(next);
        self.stack.remove(0)
    }
}

/// The RAND replacement policy, which evicts a random item.
#[derive(Default)]
pub struct Rand;

impl<I: Item> ReplacementPolicy<I> for Rand {
    fn update_state(&mut self, _: I) {}

    fn replace(&mut self, set: &HashSet<I>, _: usize, _: I) -> I {
        *set.iter()
            .choose(&mut rand::thread_rng())
            .expect("The set is non-empty.")
    }
}

/// The MRU replacement policy, which evicts the most recently used item.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::Cache;
/// use cache_sim::replacement_policy::Mru;
///
/// let mut c = Cache::<Mru>::new(3);
///
/// c.access(0);
/// c.access(1);
/// c.access(2);
/// c.access(3);
///
/// assert_eq!(c.set(), &HashSet::from([0, 1, 3]));
/// ```
#[derive(Default)]
pub struct Mru<I: Item = u32> {
    stack: Vec<I>,
}

impl<I: Item> ReplacementPolicy<I> for Mru<I> {
    fn update_state(&mut self, next: I) {
        if let Some(index) = self.stack.iter().position(|&i| i == next) {
            self.stack.remove(index);
        }

        self.stack.push(next);
    }

    fn replace(&mut self, _: &HashSet<I>, _: usize, next: I) -> I {
        self.update_state(next);

        // update_state just pushed the next item to the top of the stack, and we can't evict that
        // item (we want the most recently used item _other_ than it), so we get the second-to-last
        // item from the stack.
        self.stack.remove(self.stack.len() - 2)
    }
}

/// The LFU replacement policy, which evicts the least frequently used item.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::Cache;
/// use cache_sim::replacement_policy::Lfu;
///
/// let mut c = Cache::<Lfu>::new(3);
///
/// c.access(0);
/// c.access(0);
/// c.access(1);
/// c.access(2);
/// c.access(2);
/// c.access(3);
///
/// assert_eq!(c.set(), &HashSet::from([0, 2, 3]));
/// ```
#[derive(Default)]
pub struct Lfu<I: Item = u32> {
    counts: HashMap<I, u32>,
}

impl<I: Item> ReplacementPolicy<I> for Lfu<I> {
    fn update_state(&mut self, next: I) {
        *self.counts.entry(next).or_insert(0) += 1;
    }

    fn replace(&mut self, _: &HashSet<I>, _: usize, next: I) -> I {
        self.update_state(next);
        *self
            .counts
            .iter()
            .filter(|&(&i, _)| i != next) // we can't evict next
            .min_by_key(|&(_, &count)| count) // find the minimum count of the remaining items
            .expect("The frequency table is non-empty.")
            .0
    }
}
