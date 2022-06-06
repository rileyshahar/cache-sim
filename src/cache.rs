//! A simple demand cache simulator.

use std::collections::HashSet;
use std::fmt::Display;

use itertools::Itertools;

use crate::item::Item;
use crate::replacement_policy::ReplacementPolicy;
use crate::stats::Stat;

/// A cache, generic over a replacement policy and set of statistics.
///
/// Basic usage:
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
///
/// The cache tracks the statistics represented by the type S (default to none):
/// ```
/// use cache_sim::Cache;
/// use cache_sim::replacement_policy::Lru;
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
///
/// To track multiple statistics, use a tuple of statistics:
/// ```
/// use cache_sim::Cache;
/// use cache_sim::replacement_policy::Lru;
/// use cache_sim::stats::{HitCount, MissCount};
///
/// let mut c = Cache::<Lru, (HitCount, MissCount)>::new(3);
/// c.access(0); // miss
/// c.access(1); // miss
/// c.access(2); // miss
/// c.access(0); // hit
/// c.access(3); // miss
/// c.access(0); // hit
///
/// assert_eq!(c.stat().0.0, 2);
/// assert_eq!(c.stat().1.0, 4);
/// ```
///
pub struct Cache<R: ReplacementPolicy<I>, S: Stat<I> = (), I: Item = u32> {
    set: HashSet<I>,
    replacement_policy: R,
    capacity: usize,
    stat: S,
}

impl<R: ReplacementPolicy<I>, S: Stat<I>, I: Item> Cache<R, S, I> {
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
    pub fn access(&mut self, item: I) {
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
    pub const fn set(&self) -> &HashSet<I> {
        &self.set
    }
}

impl<R: ReplacementPolicy<I> + Default, S: Stat<I>, I: Item> Cache<R, S, I> {
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

impl<R: ReplacementPolicy<u32>, S: Stat<u32>> Cache<R, S> {
    /// If the elements in the cache are all smaller than 26, display them as letters instead.
    ///
    /// ```
    /// use cache_sim::Cache;
    /// use cache_sim::replacement_policy::Lru;
    ///
    /// // the cache's set makes no ordering guarantees, so we use capacity one to make the test
    /// // deterministic
    /// let mut c = Cache::<Lru>::new(1);
    ///
    /// c.access(0);
    /// c.access(1);
    ///
    /// assert_eq!(c.pretty_print(), "B".to_string());
    /// ```
    ///
    /// It will comma-separate as well:
    /// ```
    /// use cache_sim::Cache;
    /// use cache_sim::replacement_policy::Lru;
    ///
    /// let mut c = Cache::<Lru>::new(2);
    ///
    /// c.access(0);
    /// c.access(1);
    /// c.access(2);
    ///
    /// assert!(["B, C", "C, B"].contains(&c.pretty_print().as_str()));
    /// ```
    ///
    /// Note that this doesn't work for higher values of the item:
    /// ```
    /// use cache_sim::Cache;
    /// use cache_sim::replacement_policy::Lru;
    ///
    /// let mut c = Cache::<Lru>::new(2);
    ///
    /// c.access(25);
    /// c.access(26);
    /// c.access(0);
    ///
    /// assert!(["26, 0", "0, 26"].contains(&c.pretty_print().as_str()));
    /// ```
    #[allow(unstable_name_collisions)] // needed here, the stdlib method will do the same as the
                                       // itertools one when it's stabilized
    pub fn pretty_print(&self) -> String {
        if *self.set.iter().max().unwrap_or(&0) < 26 {
            self.set
                .iter()
                .map(|i| {
                    // treat the number as an ascii value; adding the ascii value of A so we get
                    // capital letters
                    char::from_u32(i + 'A' as u32)
                        .expect("all elements of list are valid chars")
                        .to_string()
                })
                .intersperse(", ".to_string())
                .collect()
        } else {
            self.set
                .iter()
                .map(u32::to_string)
                .intersperse(", ".to_string())
                .collect()
        }
    }
}

impl<R: ReplacementPolicy<I>, S: Stat<I>, I: Item> Display for Cache<R, S, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.set.iter().enumerate() {
            // prints the number associated with each item in the stack, in order
            if i == self.set.len() - 1 {
                write!(f, "{}", item)?;
            } else {
                write!(f, "{}, ", item)?;
            }
        }

        Ok(())
    }
}
