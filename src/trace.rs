//! A trace of accesses.

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use itertools::Itertools;

use crate::{item::Item, stats::Stat};

/// A trace.
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct Trace<I: Item = u32> {
    inner: Vec<I>,
}

impl<I: Item> From<Vec<I>> for Trace<I> {
    fn from(trace: Vec<I>) -> Self {
        Self { inner: trace }
    }
}

impl<I: Item> Trace<I> {
    /// Calculate the frequency historgram.
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use cache_sim::Trace;
    /// let frequencies = Trace::from(vec![0, 0, 1, 0, 3, 1]).frequency_histogram();
    /// assert_eq!(frequencies.get(&0), Some(&3));
    /// assert_eq!(frequencies.get(&1), Some(&2));
    /// assert_eq!(frequencies.get(&2), None);
    /// ```
    #[must_use]
    pub fn frequency_histogram(&self) -> HashMap<I, usize> {
        let mut freqs = HashMap::default();

        for &i in &self.inner {
            *freqs.entry(i).or_insert(0) += 1;
        }

        freqs
    }

    /// Calculate the stack distances.
    ///
    /// ```
    /// use cache_sim::Trace;
    ///
    /// let distances = Trace::from(vec![0, 0, 1, 0, 3, 0, 1]).stack_distances();
    /// assert_eq!(
    ///     distances.inner(),
    ///     &[None, Some(0), None, Some(1), None, Some(1), Some(2)]
    /// );
    /// ```
    ///
    /// For more details, see [`StackDistance`].
    #[must_use]
    pub fn stack_distances(&self) -> StackDistance {
        let mut distances = vec![Some(0); self.len()];

        let mut stack: Vec<&I> = Vec::new();

        for (i, curr) in self.inner().iter().enumerate() {
            if let Some(position) = stack.iter().position(|n| n == &curr) {
                // skip position + 1, then sum all the sizes until the top of the stack
                // this is our notion of size-aware stack distance, which generalizes the normal
                // version from the paging model
                distances[i] = Some(stack.iter().skip(position + 1).map(|i| i.size()).sum());
                stack.remove(position);
            } else {
                distances[i] = None;
            }
            stack.push(curr);
        }

        StackDistance { inner: distances }
    }

    pub fn iter(&self) -> std::slice::Iter<I> {
        self.inner.iter()
    }

    /// Get a reference to the inner vector of items.
    #[must_use]
    pub fn inner(&self) -> &[I] {
        self.inner.as_ref()
    }

    /// Take ownership of the inner vector of items.
    ///
    /// The ith element of the vector is the ith access of the trace.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // false positive, destructors can't be const
    pub fn into_inner(self) -> Vec<I> {
        self.inner
    }

    /// Get the length of the trace.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check whether the trace is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<I: Item> IntoIterator for Trace<I> {
    type Item = I;

    type IntoIter = <Vec<I> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'t, I: Item> IntoIterator for &'t Trace<I> {
    type Item = &'t I;

    type IntoIter = std::slice::Iter<'t, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<I: Item> FromIterator<I> for Trace<I> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self {
            inner: Vec::from_iter(iter),
        }
    }
}

impl Trace<u32> {
    /// If the elements in the trace are all smaller than 26, display them as letters instead.
    ///
    /// ```
    /// # use cache_sim::Trace;
    /// let trace = Trace::from(vec![0, 0, 2, 3, 1, 14]);
    /// assert_eq!(&trace.pretty_print(), "A, A, C, D, B, O");
    /// ```
    ///
    /// Note that this doesn't work for higher values of the item:
    /// ```
    /// # use cache_sim::Trace;
    /// let trace = Trace::from(vec![1, 2, 26]);
    /// assert_eq!(&trace.pretty_print(), "1, 2, 26");
    /// ```
    #[must_use]
    #[allow(unstable_name_collisions)] // needed here, the stdlib method will do the same as the
                                       // itertools one when it's stabilized
    pub fn pretty_print(&self) -> String {
        if *self.inner.iter().max().unwrap_or(&0) < 26 {
            self.inner
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
            self.inner
                .iter()
                .map(u32::to_string)
                .intersperse(", ".to_string())
                .collect()
        }
    }
}

impl<I: Item> Display for Trace<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.inner {
            write!(f, "{} ", i)?;
        }
        Ok(())
    }
}

impl<I: Item> Stat<I> for Trace<I> {
    fn update(&mut self, _: &std::collections::HashSet<I>, next: I, _: &HashSet<I>) {
        self.inner.push(next);
    }
}

/// The stack distances of each access in the trace.
///
/// Infinities are represented by `None`; finite distances by `Some(n)`.
///
/// ```
/// use cache_sim::Trace;
///
/// let distances = Trace::from(vec![0, 0, 1, 0, 3, 0, 1]).stack_distances();
/// assert_eq!(
///     distances.inner(),
///     &[None, Some(0), None, Some(1), None, Some(1), Some(2)]
/// );
/// ```
pub struct StackDistance {
    inner: Vec<Option<u32>>,
}

impl StackDistance {
    /// Calculate the stack distance histogram.
    ///
    /// Returns a vector of frequencies of stack distances, plus the count of intinities.
    ///
    /// ```
    /// use cache_sim::Trace;
    ///
    /// let distances = Trace::from(vec![0, 0, 1, 0, 3, 0, 1]).stack_distances();
    /// let (distance_hist, infinities) = distances.histogram();
    /// assert_eq!(distance_hist, vec![1, 2, 1]);
    /// assert_eq!(infinities, 3);
    /// ```
    pub fn histogram(&self) -> (Vec<usize>, usize) {
        let max = self.inner.iter().flatten().max();

        let mut freqs = max.map_or_else(Vec::new, |max| vec![0; *max as usize + 1]);

        let mut infinities = 0;

        for &i in &self.inner {
            #[allow(clippy::option_if_let_else)]
            if let Some(i) = i {
                freqs[i as usize] += 1;
            } else {
                infinities += 1;
            }
        }

        (freqs, infinities)
    }

    /// Get a reference to the inner vector of distances.
    ///
    /// The ith element of the vector is the ith access of the trace.
    #[must_use]
    pub fn inner(&self) -> &[Option<u32>] {
        self.inner.as_ref()
    }

    /// Take ownership of the inner vector of distances.
    ///
    /// The ith element of the vector is the ith access of the trace.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // false positive, destructors can't be const
    pub fn into_inner(self) -> Vec<Option<u32>> {
        self.inner
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
                    assert_eq!(Trace::from(vec![$($in),*]).stack_distances().inner(), vec![$($out),*])
                }
            };
        }

        stack_distance_test!(basic: 1, 2, 3 => None, None, None);
        stack_distance_test!(repeated: 1, 1, 1 => None, Some(0), Some(0));
        stack_distance_test!(one_two: 1, 2, 1, 1, 1 => None, None, Some(1), Some(0), Some(0));
        stack_distance_test!(one_repeated: 1, 2, 3, 1 => None, None, None, Some(2));
        // stack_distance_test!(empty: => );

        #[test]
        fn with_sizes() {
            use crate::item::GeneralModelGenerator;

            let mut g = GeneralModelGenerator::new();

            let a = g.item(1.0, 2);
            let b = g.item(1.0, 4);
            let c = g.item(1.0, 3);

            let trace = Trace::from(vec![a, b, c, a]);
            assert_eq!(
                trace.stack_distances().inner(),
                vec![None, None, None, Some(7)]
            );
        }
    }

    mod stack_distance_histograms {
        use super::*;

        macro_rules! stack_distance_histogram_test {
            ($name:ident: $($in:expr),* => $($out:expr),*; $infinities:expr) => {
                #[test]
                fn $name() {
                    let (freqs, infinities) = Trace::from(vec![$($in),*]).stack_distances().histogram();
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
