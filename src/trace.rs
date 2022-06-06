//! A cache trace.

use crate::item::Item;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct Trace<I: Item = u32> {
    trace: Vec<I>,
}

impl<I: Item> From<Vec<I>> for Trace<I> {
    fn from(trace: Vec<I>) -> Self {
        Self { trace }
    }
}

impl<I: Item> Trace<I> {
    /// Calculate the frequency historgram.
    ///
    /// Returns a vector of frequencies of accesses.
    #[must_use]
    pub fn frequency_histogram(&self) -> HashMap<I, usize> {
        let mut freqs = HashMap::default();

        for &i in &self.trace {
            *freqs.entry(i).or_insert(0) += 1;
        }

        freqs
    }

    /// Get a reference to the inner vector of items.
    #[must_use]
    pub fn inner(&self) -> &[I] {
        self.trace.as_ref()
    }

    /// Get the length of the trace.
    #[must_use]
    pub fn len(&self) -> usize {
        self.trace.len()
    }

    /// Check whether the trace is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.trace.is_empty()
    }
}

impl Display for Trace<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.trace.iter().max().map_or(true, |&n| n < 26) {
            for i in &self.trace {
                write!(
                    f,
                    "{}",
                    char::from_u32(i + 'A' as u32).expect("all elements of list are valid chars")
                )?;
            }
        } else {
            for i in &self.trace {
                write!(f, "{} ", i)?;
            }
        }
        Ok(())
    }
}

impl<I: Item> crate::stats::Stat<I> for Trace<I> {
    fn update(&mut self, _: &std::collections::HashSet<I>, next: I, _: Option<I>) {
        self.trace.push(next);
    }
}

pub trait Stat<I: Item> {
    fn compute(t: &Trace<I>) -> Self;
}

pub struct StackDistance {
    distances: Vec<Option<usize>>,
}

impl StackDistance {
    /// Calculate the stack distance histogram.
    ///
    /// Returns a vector of frequencies of stack distances, plus the count of intinities.
    pub fn histogram(&self) -> (Vec<usize>, usize) {
        let max = self.distances.iter().flatten().max();

        let mut freqs = max.map_or_else(Vec::new, |max| vec![0; max + 1]);

        let mut infinities = 0;

        for &i in &self.distances {
            #[allow(clippy::option_if_let_else)]
            if let Some(i) = i {
                freqs[i] += 1;
            } else {
                infinities += 1;
            }
        }

        (freqs, infinities)
    }

    #[must_use]
    pub fn distances(&self) -> &[Option<usize>] {
        self.distances.as_ref()
    }
}

impl<I: Item> Stat<I> for StackDistance {
    fn compute(t: &Trace<I>) -> Self {
        let mut distances = vec![Some(0); t.len()];

        let mut stack = Vec::new();

        for (i, curr) in t.inner().iter().enumerate() {
            let position = stack.iter().position(|n| n == &curr);
            distances[i] = position.map(|n| stack.len() - n - 1); // the stack is right-to-left
            if let Some(position) = position {
                stack.remove(position);
            }
            stack.push(curr);
        }

        Self { distances }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod stack_distance {
        use super::*;

        macro_rules! stack_distance_test {
            ($name:ident: $($in:expr),* => $($out:expr),*) => {
                #[test]
                fn $name() {
                    assert_eq!(StackDistance::compute(&Trace::from(vec![$($in),*])).distances, vec![$($out),*])
                }
            };
        }

        stack_distance_test!(basic: 1, 2, 3 => None, None, None);
        stack_distance_test!(repeated: 1, 1, 1 => None, Some(0), Some(0));
        stack_distance_test!(one_two: 1, 2, 1, 1, 1 => None, None, Some(1), Some(0), Some(0));
        stack_distance_test!(one_repeated: 1, 2, 3, 1 => None, None, None, Some(2));
        // stack_distance_test!(empty: => );
    }

    mod stack_distance_histograms {
        use super::*;

        macro_rules! stack_distance_histogram_test {
            ($name:ident: $($in:expr),* => $($out:expr),*; $infinities:expr) => {
                #[test]
                fn $name() {
                    let (freqs, infinities) = StackDistance::compute(&Trace::from(vec![$($in),*])).histogram();
                    assert_eq!(infinities, $infinities);
                    assert_eq!(freqs, vec![$($out),*]);
                }
            };
        }

        stack_distance_histogram_test!(basic: 1, 2, 3 => ; 3);
        stack_distance_histogram_test!(repeated: 1, 1, 1 => 2; 1);
        stack_distance_histogram_test!(one_two: 1, 2, 1, 1, 1 => 2, 1; 2);
        stack_distance_histogram_test!(one_repeated: 1, 2, 3, 1 => 0, 0, 1; 3);
        // stack_distance_histogram_test!(empty: => ; 0);
    }

    mod frequency {
        use super::*;

        macro_rules! frequency_test {
            ($name:ident: $($in:expr),* => $($out:expr),*) => {
                #[test]
                fn $name() {
                    assert_eq!(Trace::from(vec![$($in),*]).frequency_histogram(), HashMap::from([$($out),*]))
                }
            };
        }

        frequency_test!(basic: 1, 2, 3 => (1, 1), (2, 1), (3, 1));
        frequency_test!(repeated: 1, 1, 1 => (1, 3));
        frequency_test!(one_two: 1, 2, 1, 1, 1 => (1, 4), (2, 1));
        frequency_test!(one_repeated: 1, 2, 3, 1 => (1, 2), (2, 1), (3, 1));
        // frequency_test!(empty: => );
    }
}
