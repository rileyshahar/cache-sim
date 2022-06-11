//! Implementations of cache replacement policies.

use crate::item::{GeneralModelItem, Item};
use std::collections::{HashMap, HashSet, VecDeque};

use approx::abs_diff_eq;
use rand::seq::IteratorRandom;

/// An abstracted cache replacement policy.
pub trait ReplacementPolicy<I: Item> {
    /// Update the replacement policy's state, without evicting an item.
    fn update_state(&mut self, set: &HashSet<I>, capacity: u32, next: I);

    /// Return the item to be evicted. This should _not_ be `next`.
    fn replace(&mut self, set: &HashSet<I>, capacity: u32, next: I) -> HashSet<I>;
}

/// The LRU replacement policy, which evicts the least recently used item.
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
#[derive(Default)]
pub struct Lru<I: Item = u32> {
    stack: Vec<I>,
}

impl<I: Item> ReplacementPolicy<I> for Lru<I> {
    fn update_state(&mut self, _: &HashSet<I>, _: u32, next: I) {
        if let Some(index) = self.stack.iter().position(|&i| i == next) {
            self.stack.remove(index);
        }

        self.stack.push(next);
    }

    fn replace(&mut self, set: &HashSet<I>, capacity: u32, next: I) -> HashSet<I> {
        self.update_state(set, capacity, next);
        HashSet::from([self.stack.remove(0)])
    }
}

/// The FIFO replacement policy, which evicts the first-inserted item.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::{Cache, Fifo};
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
    stack: VecDeque<I>,
}

impl<I: Item> ReplacementPolicy<I> for Fifo<I> {
    fn update_state(&mut self, _: &HashSet<I>, _: u32, next: I) {
        if !self.stack.contains(&next) {
            self.stack.push_back(next);
        }
    }

    fn replace(&mut self, set: &HashSet<I>, capacity: u32, next: I) -> HashSet<I> {
        self.update_state(set, capacity, next);
        HashSet::from([self.stack.pop_front().expect("The cache is non-empty.")])
    }
}

/// The RAND replacement policy, which evicts a random item.
#[derive(Default)]
pub struct Rand {
    rng: rand::rngs::ThreadRng,
}

impl<I: Item> ReplacementPolicy<I> for Rand {
    fn update_state(&mut self, _: &HashSet<I>, _: u32, _: I) {}

    fn replace(&mut self, set: &HashSet<I>, _: u32, _: I) -> HashSet<I> {
        HashSet::from([*set
            .iter()
            .choose(&mut self.rng)
            .expect("The set is non-empty.")])
    }
}

/// The MRU replacement policy, which evicts the most recently used item.
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::{Cache, Mru};
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
    fn update_state(&mut self, _: &HashSet<I>, _: u32, next: I) {
        if let Some(index) = self.stack.iter().position(|&i| i == next) {
            self.stack.remove(index);
        }

        self.stack.push(next);
    }

    fn replace(&mut self, set: &HashSet<I>, capacity: u32, next: I) -> HashSet<I> {
        self.update_state(set, capacity, next);

        // update_state just pushed the next item to the top of the stack, and we can't evict that
        // item (we want the most recently used item _other_ than it), so we get the second-to-last
        // item from the stack.
        HashSet::from([self.stack.remove(self.stack.len() - 2)])
    }
}

/// The LFU replacement policy, which evicts the least frequently used item.
///
/// LFU currently does not implement any tiebreaker protocol, meaning you should not treat the
/// tiebreaker as deterministic (it's currently determined by the ordering of the underlying
/// [`HashMap`], which is not guaranteed by the language).
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::{Cache, Lfu};
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
    fn update_state(&mut self, _: &HashSet<I>, _: u32, next: I) {
        *self.counts.entry(next).or_insert(0) += 1;
    }

    fn replace(&mut self, set: &HashSet<I>, capacity: u32, next: I) -> HashSet<I> {
        self.update_state(set, capacity, next);
        HashSet::from([*self
            .counts
            .iter()
            .filter(|&(i, _)| set.contains(i)) // we have to evict something that's in the cache
            .min_by_key(|&(_, &count)| count) // find the minimum count of the remaining items
            .expect("The frequency table is non-empty.")
            .0])
    }
}

/// The landlord replacement algotihm.
///
/// Detailed in this paper: <https://arxiv.org/abs/cs/0205033>
///
/// ```
/// # use std::collections::HashSet;
/// use cache_sim::{Cache, Landlord, GeneralModelGenerator};
///
/// let mut cache = Cache::<Landlord, (), _>::new(3);
/// let mut g = GeneralModelGenerator::new();
///
/// let a = g.item(1.0, 1);
/// let b = g.item(0.5, 2);
/// let c = g.item(100.0, 2);
/// let d = g.item(1.0, 1);
///
/// cache.access(a);
/// cache.access(b);
/// cache.access(c);
/// cache.access(d);
///
/// assert_eq!(cache.set(), &HashSet::from([c, d]));
/// ```
pub struct Landlord<I: Item = GeneralModelItem> {
    credit: HashMap<I, f64>,
    credit_increase: f64,
}

impl<I: Item> Default for Landlord<I> {
    fn default() -> Self {
        Self {
            credit: HashMap::default(),
            credit_increase: 1.0,
        }
    }
}

impl<I: Item> Landlord<I> {
    /// Instantiate a new landlord replacement policy.
    ///
    /// The `credit_increase` parameter represents the percentage of the gap between the current credit
    /// and maximum credit (cost) to increase an item's credit when it is hit. It should not be above
    /// one. Higher values are closer to LRU, lower values are closer to FIFO. This defaults to 1,
    /// and should generally be between 0 and 1.
    #[must_use]
    pub fn new(credit_increase: f64) -> Self {
        Self {
            credit: HashMap::default(),
            credit_increase,
        }
    }
}

impl<I: Item> ReplacementPolicy<I> for Landlord<I> {
    fn update_state(&mut self, set: &HashSet<I>, _: u32, next: I) {
        // here we know that there is room in the cache, so we don't need to do the while loop in
        // the algorithm
        if set.contains(&next) {
            if let Some(current_credit) = self.credit.get_mut(&next) {
                *current_credit += (next.cost() - *current_credit) * self.credit_increase;
            } else {
                // should be impossible, because we know `next` is in the set.
                self.credit.insert(next, next.cost());
            }
        } else {
            self.credit.insert(next, next.cost());
        }
    }

    fn replace(&mut self, set: &HashSet<I>, capacity: u32, next: I) -> HashSet<I> {
        let mut to_evict = HashSet::default();

        while set
            .iter()
            .filter(|i| !to_evict.contains(*i))
            .map(Item::size)
            .sum::<u32>()
            + next.size()
            > capacity
        {
            // have to compute min cost by hand because of limitations with float
            let mut current_min_cost = f64::MAX;
            let mut current_min_item = None;
            for item in set {
                if item.cost() < current_min_cost {
                    current_min_cost = item.cost();
                    current_min_item = Some(item);
                }
            }

            let min = current_min_item.expect("The set is non-empty.");
            let delta =
                self.credit.get(min).expect("The item is in the set.") / f64::from(min.size());

            // decrease the credit for everything in the set
            for item in set {
                *self.credit.get_mut(item).expect("The item is in the set.") -=
                    delta * f64::from(item.size());
            }

            // evict items with no credit
            to_evict.extend(set.iter().filter(|i| {
                abs_diff_eq!(self.credit.get(i).expect("The item is in the set."), &0.0)
            }));
        }

        self.update_state(set, capacity, next);

        to_evict
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cache;

    macro_rules! integration_test {
        ($name:ident ($policy:ty): $($in:expr),* => $($out:expr),*) => {
            #[test]
            fn $name() {
                let mut c = Cache::<$policy>::new(3);

                $(
                    c.access($in);
                )*

                assert_eq!(c.set(), &HashSet::from([$($out),*]));
            }
        };
    }

    macro_rules! replacement_policy_test {
        ($name:ident ($policy:ty):
            counting_up => $($counting_up_out:expr),*;
            repeated => $($repeated_out:expr),*;
            one_repetition => $($one_repetition_out:expr),*;
            cycle => $($cycle_out:expr),*;
        ) => {
            mod $name {
                use super::*;

                integration_test!(counting_up ($policy): 0, 1, 2, 3 => $($counting_up_out),*);
                integration_test!(repeated ($policy): 0, 0, 0, 0 =>  $($repeated_out),*);
                integration_test!(one_repetition ($policy): 0, 1, 2, 0, 3 => $($one_repetition_out),*);
                integration_test!(cycle ($policy): 0, 1, 2, 0, 1, 2, 0, 1, 2, 3, 3 => $($cycle_out),*);
            }
        }
    }

    replacement_policy_test! {
        lru (Lru):
            counting_up => 1, 2, 3;
            repeated => 0;
            one_repetition => 0, 2, 3;
            cycle => 1, 2, 3;
    }

    replacement_policy_test! {
        mru (Mru):
            counting_up => 0, 1, 3;
            repeated => 0;
            one_repetition => 1, 2, 3;
            cycle => 0, 1, 3;
    }

    replacement_policy_test! {
        fifo (Fifo):
            counting_up => 1, 2, 3;
            repeated => 0;
            one_repetition => 1, 2, 3;
            cycle => 1, 2, 3;
    }

    // mod lru {
    //     use super::*;

    //     macro_rules! lru_integration_test {
    //         ($name:ident: $($in:expr),* => $($out:expr),*) => {
    //             #[test]
    //             fn $name() {
    //                 let mut c = Cache::<Lru>::new(3);

    //                 $(
    //                     c.access($in);
    //                 )*

    //                 assert_eq!(c.set(), &HashSet::from([$($out),*]));
    //             }
    //         };
    //     }

    //     lru_integration_test!(counting_up: 0, 1, 2, 3 => 1, 2, 3);
    //     lru_integration_test!(repeated: 0, 0, 0, 0 => 0);
    //     lru_integration_test!(one_repetition: 0, 1, 2, 0, 3 => 0, 2, 3);
    //     lru_integration_test!(cycle: 0, 1, 2, 0, 1, 2, 0, 1, 2, 3, 3 => 1, 2, 3);
    // }
}
