//! Contains utilities for managing a cache.

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct Item(pub u32); // TODO: figure out what this should be represented as

/// A cache, generic over a replacement policy.
pub struct Cache<R: ReplacementPolicy> {
    /// The stack. Lower indices are present in smaller caches.
    ///
    /// TODO: is a vector the right data structure
    stack: Vec<Item>,
    replacement_policy: R,
}

impl<R: ReplacementPolicy> Cache<R> {
    /// Create an empty cache using the replacement policy.
    pub fn new(replacement_policy: R) -> Self {
        Self {
            stack: Vec::default(),
            replacement_policy,
        }
    }

    /// Update the cache after an access to item.
    pub fn access(&mut self, item: Item) {
        let stack = std::mem::take(&mut self.stack);
        self.stack = self.replacement_policy.replace(stack, item);
    }
}

// An implementation of printing a cache.
impl<R: ReplacementPolicy> std::fmt::Display for Cache<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.stack.iter().max().map_or(0, |i| i.0) < 26 {
            for item in &self.stack {
                // prints the letter of each item, i.e. 0 -> A, 1 -> B, etc
                write!(
                    f,
                    "{}",
                    char::from_u32(item.0 + 'A' as u32)
                        .expect("all elements of list are valid chars")
                )?;
            }
        } else {
            for item in &self.stack {
                // prints the number associated with each item in the stack, in order
                write!(f, "{}", item.0)?;
            }
        }

        Ok(())
    }
}

pub trait ReplacementPolicy {
    /// Return the updated stack after an access to next, possibly modifying internal state.
    fn replace(&mut self, stack: Vec<Item>, next: Item) -> Vec<Item>;
}

/// The LRU replacement policy.
pub struct Lru;

impl ReplacementPolicy for Lru {
    /// An implementation of the `replace` method for the trait.
    fn replace(&mut self, mut stack: Vec<Item>, next: Item) -> Vec<Item> {
        if let Some(index) = stack.iter().position(|&i| i == next) {
            // remove the accessed item from its current place in the stack
            stack.remove(index);
        }
        // insert it to the front of the stack, since it's least recently used
        stack.insert(0, next);

        stack
    }
}
