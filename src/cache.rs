//! Contains utilities for managing a cache.

use std::collections::HashSet;

use super::replacement_policy::ReplacementPolicy;
use super::Item;

/// A cache, generic over a replacement policy.
pub struct Cache<R: ReplacementPolicy> {
    set: HashSet<Item>,
    replacement_policy: R,
    capacity: usize,
}

impl<R: ReplacementPolicy> Cache<R> {
    /// Create an empty cache using the replacement policy.
    pub fn new(replacement_policy: R, capacity: usize) -> Self {
        Self {
            set: HashSet::default(),
            replacement_policy,
            capacity,
        }
    }

    /// Update the cache after an access to item.
    pub fn access(&mut self, item: Item) {
        // we always need to run this, even if not at capacity, so that the replacement policy can
        // update its state
        let to_evict = self
            .replacement_policy
            .replace(&self.set, self.capacity, item);
        if let Some(to_evict) = to_evict {
            self.set.remove(&to_evict);
        }
        self.set.insert(item);
    }
}

// An implementation of printing a cache.
impl<R: ReplacementPolicy> std::fmt::Display for Cache<R> {
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
            for item in &self.set {
                // prints the number associated with each item in the stack, in order
                write!(f, "{}", item.0)?;
            }
        }

        Ok(())
    }
}
