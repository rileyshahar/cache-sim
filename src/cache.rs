//! Contains utilities for managing a cache.

use std::collections::HashSet;

use super::replacement_policy::ReplacementPolicy;
use super::stats::Stat;
use super::Item;

/// A cache, generic over a replacement policy.
pub struct Cache<R: ReplacementPolicy, S: Stat> {
    set: HashSet<Item>,
    replacement_policy: R,
    capacity: usize,
    stat: S,
}

impl<R: ReplacementPolicy, S: Stat> Cache<R, S> {
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
        // we always need to run this, even if not at capacity, so that the replacement policy can
        // update its state
        if self.set.len() >= self.capacity && !self.set.contains(&item) {
            let to_evict = self
                .replacement_policy
                .replace(&self.set, self.capacity, item);
            self.stat.update(&self.set, item, Some(to_evict));
            self.set.remove(&to_evict);
        } else {
            self.replacement_policy.update_state(item);
            self.stat.update(&self.set, item, None);
        }
        self.set.insert(item);
    }

    /// Get a reference to cache's statistic.
    pub const fn stat(&self) -> &S {
        &self.stat
    }
}

// An implementation of printing a cache.
impl<R: ReplacementPolicy, S: Stat> std::fmt::Display for Cache<R, S> {
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
