//! A simple demand cache simulator.

use std::collections::HashSet;
use std::fmt::Display;

use itertools::Itertools;

use crate::item::Item;
use crate::replacement_policy::ReplacementPolicy;
use crate::stats::Stat;
use crate::trace::Trace;

/// A cache, generic over a replacement policy and set of statistics.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::{Cache, Lru};
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
pub struct Cache<R: ReplacementPolicy<I>, S: Stat<I> = (), I: Item = u32> {
    set: HashSet<I>,
    replacement_policy: R,
    capacity: f64,
    stat: S,
}

impl<R: ReplacementPolicy<I>, S: Stat<I>, I: Item> Cache<R, S, I> {
    /// Create an empty cache using an explicitly configured replacement policy.
    pub fn with_replacement_policy(policy: R, capacity: impl Into<f64>) -> Self {
        Self {
            set: HashSet::default(),
            replacement_policy: policy,
            capacity: capacity.into(),
            stat: S::default(),
        }
    }

    /// Get the currently used capacity of the set of items.
    fn used_capacity(&self) -> f64 {
        self.set.iter().map(Item::size).sum()
    }

    /// Check whether the cache has space for item.
    fn has_capacity_for(&self, item: I) -> bool {
        self.used_capacity() + item.size() <= self.capacity
    }

    /// Update the cache after an access to item.
    ///
    /// # Panics
    ///
    /// If the replacement policy errors, and so we end up over capacity.
    pub fn access(&mut self, item: I) {
        if self.set.contains(&item) || self.has_capacity_for(item) {
            // we're assuming demand caching for now, so here we don't need to change anything in
            // the cache, and we just update the state of the replacement policy and the statistics
            self.replacement_policy
                .update_state(&self.set, self.capacity, item);
            self.stat.update(&self.set, item, &HashSet::new());
        } else {
            // here we actually need to evict something
            let to_evict = self
                .replacement_policy
                .replace(&self.set, self.capacity, item);

            self.stat.update(&self.set, item, &to_evict);

            // TODO: is there an easy restructuring of this that prevents us from evicting and then
            // reinserting `item`, thus ending with an over capacity cache? This can happen now if
            // the replacement policy is implemented incorrectly.
            for item in to_evict {
                self.set.remove(&item);
            }
        }

        // finally, again because we assume demand paging, we always have to put the last access
        // into the cache
        self.set.insert(item);

        assert!(self.capacity >= self.used_capacity());
    }

    /// Update the cache after accessing all items in the trace.
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// use cache_sim::{Cache, Lru, Trace};
    ///
    /// let mut c = Cache::<Lru>::new(3);
    /// let t = Trace::from(vec![0, 1, 2, 0, 3]);
    ///
    /// c.run_trace(&t);
    ///
    /// assert_eq!(c.set(), &HashSet::from([0, 2, 3]));
    /// ```
    pub fn run_trace(&mut self, trace: &Trace<I>) {
        for item in trace {
            self.access(*item);
        }
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
    pub fn new(capacity: impl Into<f64>) -> Self {
        Self {
            set: HashSet::default(),
            replacement_policy: R::default(),
            capacity: capacity.into(),
            stat: S::default(),
        }
    }
}

impl<R: ReplacementPolicy<u32>, S: Stat<u32>> Cache<R, S> {
    /// If the elements in the cache are all smaller than 26, display them as letters instead.
    ///
    /// ```
    /// # use cache_sim::{Cache, Lru};
    /// // the cache's set makes no ordering guarantees, so we use capacity one to make the test
    /// // deterministic
    /// let mut c = Cache::<Lru>::new(1);
    ///
    /// c.access(0);
    /// c.access(1);
    ///
    /// assert_eq!(&c.pretty_print(), "B");
    /// ```
    ///
    /// It will comma-separate as well:
    /// ```
    /// # use cache_sim::{Cache, Lru};
    /// let mut c = Cache::<Lru>::new(2);
    ///
    /// c.access(0);
    /// c.access(1);
    /// c.access(2);
    ///
    /// let pretty_print = c.pretty_print();
    /// assert!("B, C" == pretty_print || "C, B" == pretty_print);
    /// ```
    ///
    /// Note that this doesn't work for higher values of the item:
    /// ```
    /// # use cache_sim::Cache;
    /// # use cache_sim::Lru;
    /// let mut c = Cache::<Lru>::new(2);
    ///
    /// c.access(25);
    /// c.access(26);
    /// c.access(0);
    ///
    /// let pretty_print = c.pretty_print();
    /// assert!("0, 26" == pretty_print || "26, 0" == pretty_print);
    /// ```
    #[must_use]
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
