//! Constains replacement policies.

use super::Item;
use std::collections::{HashMap, HashSet};

use rand::seq::IteratorRandom;

/// An abstracted cache replacement policy.
pub trait ReplacementPolicy {
    /// Update the replacement policy's state, without evicting an item.
    fn update_state(&mut self, next: Item);

    /// Return the item to be evicted. This should _not_ be `next`.
    fn replace(&mut self, set: &HashSet<Item>, capacity: usize, next: Item) -> Item;
}

/// The LRU replacement policy, which evicts the least recently used item.
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

/// The FIFO replacement policy, which evicts the first-inserted item.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::{Cache, Item};
/// use cache_sim::replacement_policy::Fifo;
///
/// let mut c = Cache::<Fifo>::new(3);
///
/// c.access(Item(0));
/// c.access(Item(1));
/// c.access(Item(2));
/// c.access(Item(0));
/// c.access(Item(3));
///
/// assert_eq!(c.set(), &HashSet::from([Item(1), Item(2), Item(3)]));
/// ```
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

/// The RAND replacement policy, which evicts a random item.
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

/// The MRU replacement policy, which evicts the most recently used item.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::{Cache, Item};
/// use cache_sim::replacement_policy::Mru;
///
/// let mut c = Cache::<Mru>::new(3);
///
/// c.access(Item(0));
/// c.access(Item(1));
/// c.access(Item(2));
/// c.access(Item(3));
///
/// assert_eq!(c.set(), &HashSet::from([Item(0), Item(1), Item(3)]));
/// ```
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
/// use cache_sim::{Cache, Item};
/// use cache_sim::replacement_policy::Lfu;
///
/// let mut c = Cache::<Lfu>::new(3);
///
/// c.access(Item(0));
/// c.access(Item(0));
/// c.access(Item(1));
/// c.access(Item(2));
/// c.access(Item(2));
/// c.access(Item(3));
///
/// assert_eq!(c.set(), &HashSet::from([Item(0), Item(2), Item(3)]));
/// ```
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
            .filter(|&(&i, _)| i != next) // we can't evict next
            .min_by_key(|&(_, &count)| count) // find the minimum count of the remaining items
            .expect("The frequency table is non-empty.")
            .0
    }
}
