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
            trace[(index - self.items.len())..index] == self.items
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

// we need to implement this so we can call methods that expct `Condition` on type-erased condition
// trait objects; see `Trace::write_conditional_frequencies` for a motivating example.
//
// TODO: is there a non terrible way to solve this
impl<I: Item> Condition<I> for Box<dyn Condition<I>> {
    fn check(&self, trace: &Trace<I>, index: usize) -> bool {
        self.as_ref().check(trace, index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod no_condition {
        use super::*;

        macro_rules! test_case {
            // don't need to take an output, since no condition should always return true
            ( $name:ident: $($in:expr),*; $index:expr ) => {
                #[test]
                fn $name() {
                    assert!(NoCondition.check(&Trace::from(vec![$($in),*]), $index));
                }
            };
        }

        test_case!(counting_up: 1, 2, 3; 0);
        test_case!(zeros: 0, 0, 0; 2);
        test_case!(noise: 1, 0, 5, 3, 17; 3);
    }

    mod closure_condition {
        use super::*;

        macro_rules! test_case {
            ( $name:ident: $condition:expr, on $($in:expr),*; $index:expr => $out:expr ) => {
                #[test]
                fn $name() {
                    assert_eq!($condition.check(&Trace::from(vec![$($in),*]), $index), $out);
                }
            }
        }

        test_case!(equals_zero: |t: &Trace<_>, i| t[i] == 0, on 1, 2, 0; 2 => true);
        test_case!(after_one: |t: &Trace<_>, i| i>0 && t[i-1] == 1, on 1, 0, 1, 2; 3 => true);
        test_case!(one_greater: |t: &Trace<_>, i| i>0 && t[i-1] + 1 == t[i], on 0, 2, 0, 1; 3 => true);
        test_case!(wrong_condition: |t: &Trace<_>, i| i>0 && t[i-1] + 1 == t[i], on 0, 2, 0, 1; 2 => false);
    }

    mod last_n_condition {
        use super::*;

        macro_rules! test_case {
            ( $name:ident: $($seq:expr),*; on $($in:expr),*; $index:expr => $out:expr ) => {
                #[test]
                fn $name() {
					let condition = LastNItems::new(vec![$($seq),*]);
                    assert_eq!(condition.check(&Trace::from(vec![$($in),*]), $index), $out);
                }
            }
        }

        test_case!(one_zero: 0; on 1, 0, 2; 2 => true);
        test_case!(incrementing: 1, 2; on 0, 1, 2, 3; 3 => true);
        test_case!(repeated: 1; on 1, 2, 1, 0; 3 => true);
        test_case!(wrong_condition: 3; on 1, 2, 0, 1; 2 => false);
    }
}
