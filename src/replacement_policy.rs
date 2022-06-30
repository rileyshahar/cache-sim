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

pub trait Tiebreaker<I: Item>: ReplacementPolicy<I> {
    /// Pick a single item to evict.
    fn tiebreak(&mut self, from: &HashSet<I>, size_to_free: u32) -> HashSet<I>;
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

impl<I: Item> Tiebreaker<I> for Lru<I> {
    fn tiebreak(&mut self, from: &HashSet<I>, size_to_free: u32) -> HashSet<I> {
        let mut ret = HashSet::new();

        while size_to_free > ret.iter().map(Item::size).sum() && ret.len() < from.len() {
            ret.extend(
                self.stack
                    .iter()
                    .filter(|&i| !ret.contains(i))
                    .find(|i| from.contains(i)),
            );
        }

        assert!(!ret.is_empty());
        ret
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
/// The tiebreaker defaults to Lru.
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
pub struct Lfu<I: Item = u32, T: Tiebreaker<I> = Lru> {
    counts: HashMap<I, u32>,
    tiebreaker: T,
}

impl<I: Item, T: Tiebreaker<I>> ReplacementPolicy<I> for Lfu<I, T> {
    fn update_state(&mut self, set: &HashSet<I>, capacity: u32, next: I) {
        *self.counts.entry(next).or_insert(0) += 1;
        self.tiebreaker.update_state(set, capacity, next);
    }

    fn replace(&mut self, set: &HashSet<I>, capacity: u32, next: I) -> HashSet<I> {
        self.update_state(set, capacity, next);
        let min = self
            .counts
            .iter()
            .filter(|&(i, _)| set.contains(i)) // we have to evict something that's in the cache
            .map(|(_, &count)| count)
            .min()
            .expect("The set is non-empty.");

        self.tiebreaker.tiebreak(
            &self
                .counts
                .iter()
                .filter(|&(_, &count)| count == min)
                .map(|(&i, _)| i)
                .collect(),
            1,
        )
    }
}

/// The landlord replacement algotihm.
///
/// Detailed in this paper: <https://arxiv.org/abs/cs/0205033>
///
/// The tiebreaker (for evicting multiple zero-credit items) defaults to Lru.
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
pub struct Landlord<I: Item = GeneralModelItem, T: Tiebreaker<I> = Lru<GeneralModelItem>> {
    credit: HashMap<I, f64>,
    credit_increase: f64,
    tiebreaker: T,
}

impl<I: Item, T: Tiebreaker<I> + Default> Default for Landlord<I, T> {
    fn default() -> Self {
        Self {
            credit: HashMap::default(),
            credit_increase: 1.0,
            tiebreaker: T::default(),
        }
    }
}

impl<I: Item, T: Tiebreaker<I> + Default> Landlord<I, T> {
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
            tiebreaker: T::default(),
        }
    }
}

impl<I: Item, T: Tiebreaker<I>> Landlord<I, T> {
    /// Instantiate a new landlord replacement policy, with a specifically configured tiebreaker.
    ///
    /// The `credit_increase` parameter represents the percentage of the gap between the current credit
    /// and maximum credit (cost) to increase an item's credit when it is hit. It should not be above
    /// one. Higher values are closer to LRU, lower values are closer to FIFO. This defaults to 1,
    /// and should generally be between 0 and 1.
    #[must_use]
    pub fn with_tiebreaker(tiebreaker: T, credit_increase: f64) -> Self {
        Self {
            credit: HashMap::default(),
            credit_increase,
            tiebreaker,
        }
    }
}

impl<I: Item, T: Tiebreaker<I>> ReplacementPolicy<I> for Landlord<I, T> {
    fn update_state(&mut self, set: &HashSet<I>, capacity: u32, next: I) {
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

        self.tiebreaker.update_state(set, capacity, next);
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
            let mut current_delta = f64::MAX;
            let mut current_min_item = None;
            for item in set {
                let item_delta = *self
                    .credit
                    .get(item)
                    .expect("Items in the set have a credit.")
                    / f64::from(item.size());
                if item_delta < current_delta {
                    current_delta = item_delta;
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
            to_evict.extend(
                self.tiebreaker.tiebreak(
                    &set.iter()
                        .filter(|&i| !to_evict.contains(i))
                        .filter(|i| {
                            abs_diff_eq!(self.credit.get(i).expect("The item is in the set."), &0.0)
                        })
                        .copied()
                        .collect(),
                    set.iter()
                        .filter(|i| !to_evict.contains(*i))
                        .map(Item::size)
                        .sum::<u32>()
                        + next.size()
                        - capacity,
                ),
            );
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

    mod landlord {
        use super::*;
        use crate::GeneralModelGenerator;

        #[test]
        fn lru_tiebreaker() {
            let mut cache = Cache::<Landlord, (), _>::new(3);
            let mut gen = GeneralModelGenerator::new();

            let a = gen.item(1.0, 1);
            let b = gen.item(2.0, 2);
            let c = gen.item(1.0, 1);

            cache.access(a);
            cache.access(b);
            cache.access(c);

            // should evict a because LRU
            assert_eq!(cache.set(), &HashSet::from([b, c]));
        }

        #[test]
        fn credit_not_cost() {
            // we had a bug where we would compute delta with the cost, not the credit. this tests
            // that that is fixed.
            let mut cache = Cache::<Landlord, (), _>::new(3);
            let mut gen = GeneralModelGenerator::new();

            let itm_a = gen.item(2.0, 1);
            let itm_b = gen.item(3.0, 1);
            let itm_c = gen.item(3.0, 1);
            let itm_z = gen.item(2.0, 1);
            let itm_d = gen.item(1.0, 1);

            cache.access(itm_a);
            cache.access(itm_b);
            cache.access(itm_c);
            cache.access(itm_z);
            cache.access(itm_d);
            cache.access(itm_a);

            assert_eq!(cache.set(), &HashSet::from([itm_a, itm_d, itm_z]));
        }
    }
}
