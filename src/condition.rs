//! Implementations of conditions for filtering the frequency histogram
use crate::item::Item;
use crate::trace::Trace;

/// An abstract representation of a condition on a frequency histogram.
pub trait Condition<I: Item> {
    /// Check whether a certain element should be counted by the histogram.
    /// Needs the trace and the index of the element to check.
    fn check(&self, trace: &Trace<I>, index: usize) -> bool;
}

/// No condition on the trace, includes all elements for a full frequency histogram
#[derive(Default, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct NoCondition;

impl<I: Item> Condition<I> for NoCondition {
    //always returns true
    fn check(&self, _: &Trace<I>, _: usize) -> bool {
        true
    }
}

/// A general condition for checking the previous N items.  Can take any vector of items
/// and filters for accesses in the trace where the last N items were the exact contents
/// of the vector.
#[derive(Default, Debug)]
pub struct LastNItems<I: Item> {
    items: Vec<I>,
}

impl<I: Item> LastNItems<I> {
    #[must_use]
    pub fn new(items: Vec<I>) -> Self {
        Self { items }
    }
}

impl<I: Item> Condition<I> for LastNItems<I> {
    fn check(&self, trace: &Trace<I>, index: usize) -> bool {
        if index >= self.items.len() {
            trace.inner()[(index - self.items.len())..index].to_vec() == self.items
        } else {
            false
        }
    }
}

impl<I: Item, F: Fn(&Trace<I>, usize) -> bool> Condition<I> for F {
    fn check(&self, trace: &Trace<I>, index: usize) -> bool {
        self(trace, index)
    }
}
