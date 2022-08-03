//! A trace of accesses.

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use itertools::Itertools;

use crate::output::histogram_out;
use crate::output::write_header;
use crate::{condition::Condition, item::Item, stats::Stat};

/// A trace.
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct Trace<I: Item = u32> {
    inner: Vec<I>,
    strides: Vec<i64>,
}

impl<I: Item> From<Vec<I>> for Trace<I> {
    fn from(trace: Vec<I>) -> Self {
		let strides = trace_strides::<I>(&trace);
        Self { inner: trace , strides}
    }
}

impl<I: Item> Trace<I> {
    /// Calculate the frequency histogram of the items based on a given condition.
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use cache_sim::{Trace, NoCondition};
    /// let frequencies = Trace::from(vec![0, 0, 1, 0, 3, 1]).frequency_histogram(&NoCondition);
    /// assert_eq!(frequencies.get(&0), Some(&3));
    /// assert_eq!(frequencies.get(&1), Some(&2));
    /// assert_eq!(frequencies.get(&2), None);
    /// ```
    #[must_use]
    pub fn frequency_histogram(&self, condition: &impl Condition<I>) -> HashMap<I, u32> {
        let mut freqs = HashMap::default();
		
        for i in 0..self.inner.len() {
            if condition.check(self, i) {
                *freqs.entry(self.inner[i]).or_insert(0) += 1;
            }
        }

        freqs
    }
    
    /// Calculate the frequency histogram of the strides based on a given condition.
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use cache_sim::{Trace, NoCondition};
    /// let strides = Trace::from(vec![0, 0, 4, 3, 2, 2, 0]).stride_histogram(&NoCondition);
    /// assert_eq!(frequencies.get(&0), Some(&2));
    /// assert_eq!(frequencies.get(&-1), Some(&2));
    /// assert_eq!(frequencies.get(&3), None);
    /// ```
    pub fn stride_histogram(&self, condition: &impl Condition<I>) -> HashMap<i64, u32> {
        let mut freqs = HashMap::default();
		
        for i in 0..self.strides.len() {
            if condition.check(self, i) {
                *freqs.entry(self.strides[i]).or_insert(0) += 1;
            }
        }

        freqs
    }
    
    /// Calculate several frequency histograms, each identified by a string.  Can produce a
	/// mix of item and stride frequency histograms, determined by the bool accompanying the condition.
	/// (`true` would indicate a stride frequency histogram)
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use cache_sim::{Trace, NoCondition};
	/// let mut conditions: HashMap<String, (Box<dyn Condition<GeneralModelItem>>,bool)> = HashMap::with_capacity(2);
	///	conditions.insert(String::from("Items"), (Box::new(NoCondition),false));
	/// conditions.insert(String::from("Strides"), (Box::new(NoCondition),true));
    /// let histograms = Trace::from(vec![0, 0, 4, 3, 2, 2, 0]).frequency_histogram_many(&conditions);
    /// assert_eq!(histograms.get(&"Items").get(&0), Some(&3));
    /// assert_eq!(frequencies.get(&"Strides").get(&0), Some(&1));
    /// ```
    pub fn frequency_histogram_many<'a>(&self, conditions: &'a HashMap<String, (Box<dyn Condition<I>>, bool)>) -> (HashMap<&'a str, HashMap<I, u32>>,HashMap<&'a str, HashMap<i64, u32>>) {
        let mut dists = HashMap::default();
        let mut dists2 = HashMap::default();

        for i in 0..self.inner.len() {
			for (name, (condition, use_stride)) in conditions{
            	if !use_stride && condition.check(self, i) {
					//TODO: is this a problem? Do we need to check for none here?
                	*dists.entry(name.as_str()).or_insert(HashMap::default()).entry(self.inner[i]).or_insert(0) += 1;
            	}
            	else if *use_stride && i < self.strides.len() && condition.check(self, i) {
					*dists2.entry(name.as_str()).or_insert(HashMap::default()).entry(self.strides[i]).or_insert(0) += 1;
				}
            }
        }

        (dists,dists2)
    }
    

    /// Calculate the stack distances.  Under the paging model, size is considered 1 for all items.
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
    pub fn stack_distances(&self, paging_model: bool) -> StackDistance {
        let mut distances = vec![Some(0); self.len()];

        let mut stack: Vec<&I> = Vec::new();

        for (i, curr) in self.iter().enumerate() {
            if let Some(position) = stack.iter().position(|n| n == &curr) {
                // skip position + 1, then sum all the sizes until the top of the stack
                // this is our notion of size-aware stack distance, which generalizes the normal
                // version from the paging model
                if paging_model {
                    distances[i] = Some(stack.iter().skip(position + 1).count() as u32);
                }
                else{
                	distances[i] = Some(stack.iter().skip(position + 1).map(|i| i.size()).fold(0, |sum,val| if sum < 1000000000 {sum + val} else{sum}));
                }
                stack.remove(position);
            } else {
                distances[i] = None;
            }
            stack.push(curr);
        }

        StackDistance { inner: distances }
    }

    /// Write the conditional frequencies for each condition to the output stream.
    ///
    /// Writer is a function that can give us a writer; ideally it should return a handle to the
    /// same underlying output stream each time.
    ///
    /// # Errors
    /// If writing to the csv fails.
    ///
    /// TODO: figure out a non-boxed return type
    pub fn write_conditional_frequencies<W: std::io::Write>(
        &self,
        conditions: HashMap<String, (Box<dyn Condition<I>>,bool)>,
        writer: impl Fn() -> anyhow::Result<W>,
    ) -> anyhow::Result<()> {
        // TODO: update this if we write a more efficient way to get frequencies for different
        // conditions
		
		//write header row
		let labels = vec![String::from("Name"),String::from("Entropy"),String::from("Data")];
		write_header(&labels,writer()?)?;
		
		let histograms = self.frequency_histogram_many(&conditions);
		
		dbg!("histograms made");
        for (name, histogram) in histograms.0 {
            histogram_out(name, entropy(&histogram), &histogram, writer()?)?;
        }
        for (name, histogram) in histograms.1 {
            histogram_out(name, entropy(&histogram), &histogram, writer()?)?;
        }
        /*
        
        for (name, condition) in conditions {
			let histogram = self.frequency_histogram(&condition);
            histogram_out(&name, entropy(&histogram), &histogram, &items, writer()?)?;
        }
		*/
        Ok(())
    }
    
    /// Calculates the conditional entropy of an item, conditioned on the last `prefix` items seen.
	/// 
	/// Each sequence of items has a distribution of the items that follow it, and this is the weighted
	/// sum of all of those.
    pub fn average_entropy(&self, prefix: usize) -> f64{
		//calculates its own frequencies rather than relying on frequency_histogram for performance reasons
		let mut freqs: HashMap<&[I], u32> = HashMap::default();
		let mut distributions: HashMap<&[I], HashMap<I, u32>> = HashMap::default();
		dbg!("counting items...");
        for i in prefix..self.inner.len() {
        	*freqs.entry(&self.inner[(i-prefix)..i]).or_insert(0) += 1;
			*distributions.entry(&self.inner[(i-prefix)..i]).or_insert(HashMap::default()).entry(self.inner[i]).or_insert(0) += 1;
        }
        dbg!("item freqs done");
		let mut sum: f64 = 0.0;
		//TODO: this also looks slow - can we speed it up somehow?
		for (seq,count) in freqs{
			if let Some(hist) = distributions.get(&seq){
				sum += ((count as f64)/((self.len()-prefix) as f64))*entropy(hist);
			}
		}
		dbg!("item entropy done");
		sum
	}
	
	/// Calculates the conditional entropy of a stride length, conditioned on the last `prefix` strides seen.
	/// 
	/// Each sequence of items has a distribution of the strides that follow it, and this is the weighted
	/// sum of all of those.
	/// 
	/// (This is analagous to `average_entropy` for items)
	pub fn stride_entropy(&self, prefix: usize) -> f64{
		//almost identical to average_entropy
		let mut freqs: HashMap<&[i64], u32> = HashMap::default();
		let mut distributions: HashMap<&[i64], HashMap<i64, u32>> = HashMap::default();
		dbg!("counting strides...");
        for i in prefix..self.strides.len() {
        	*freqs.entry(&self.strides[(i-prefix)..i]).or_insert(0) += 1;
			*distributions.entry(&self.strides[(i-prefix)..i]).or_insert(HashMap::default()).entry(self.strides[i]).or_insert(0) += 1;
        }
        dbg!("stride freqs done");
		let mut sum: f64 = 0.0;
		//this also looks slow - can we speed it up somehow
		for (seq,count) in freqs{
			if let Some(hist) = distributions.get(&seq){
				sum += ((count as f64)/((self.len()-prefix) as f64))*entropy(hist);
			}
		}
		dbg!("stride entropy done");
		sum
	}

	/// Get an iterator over the inner vector of items
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
    
    /// Get a reference to the inner vector of strides.
    pub fn strides(&self) -> &[i64] {
        self.strides.as_ref()
    }

    /// Get the length of the trace.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Get the number of unique sequences of items in the trace that are exactly `length` long
	pub fn num_items(&self, length: usize) -> usize {
		let mut seqs = HashSet::<Vec<u64>>::new();
        for i in 0..(self.inner.len() - length){
			seqs.insert(self.inner[i..i+length].iter().map(|&i| i.id()).collect());
		}
		seqs.len()
    }
    
    /// Get the number of unique sequences of strides in the trace that are exactly `length` long
	pub fn num_strides(&self, length: usize) -> usize {
        let mut seqs = HashSet::<Vec<i64>>::new();
        for i in 0..(self.strides.len() - length){
			seqs.insert(self.strides[i..i+length].to_vec());
		}
		seqs.len()
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
		let vector_form = Vec::from_iter(iter);
		let strides = trace_strides::<I>(&vector_form);
        Self {
            inner: vector_form,
            strides,
        }
    }
}

// Allows indexing the trace with any type that could index the underlying vector, e.x. with usizes
// or `Range`s from the standard library.
impl<I: Item, Idx> std::ops::Index<Idx> for Trace<I>
where
    Idx: std::slice::SliceIndex<[I]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.inner[index]
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

/// Returns the entropy of a given distribution.
#[must_use]
pub fn entropy<I: Item, H: std::hash::BuildHasher>(histogram: &HashMap<I, u32, H>) -> f64 {
    let total = f64::from(histogram.values().sum::<u32>());
    -histogram
        .values()
        .map(|&i| (f64::from(i) / total) * ((f64::from(i) / total).log2()))
        .sum::<f64>()
}


/// Calculates the entropy of the strides involved in runs of several of the same stride.
/// 
/// `cont` indicates how far to look ahead, where `1` means you include the current stride.
/// For most purposes, `cont` should be either `0` or `1`.  `cont` does not affect how far back
/// to look; i.e. this function looks for stride sequences of length `prefix + cont` in total.
pub fn linear_function_entropy<I: Item>(trace: &Trace<I>, prefix: usize, cont: usize) -> f64{
	let freqs = trace.stride_histogram(&|t: &Trace<I>, i: usize| i >= prefix && 
	t.strides()[i-prefix..i+cont].iter().fold((true,t.strides()[i-prefix]) , |(state,last),&next| (state && last == next,next)).0);
	
	entropy(&freqs)
}

/// Calculates the entropy of the items involved in exponential sequences of accesses, i.e. a sequence where the
/// offset doubles each access.  Not very useful.
/// 
/// `cont` indicates how far to look ahead, where `1` means you include the current item.
/// For most purposes, `cont` should be either `0` or `1`.  `cont` does not affect how far back
/// to look; i.e. this function looks for exponential sequences of length `prefix + cont` in total.
pub fn exp_function_entropy<I: Item>(trace: &Trace<I>, prefix: usize, cont: usize) -> f64{
	let freqs = trace.frequency_histogram(&|t: &Trace<I>, i: usize| i > prefix && t[i-prefix+1..i+cont].iter()
	.fold((&t[i-prefix],(t[i-prefix].id() as f64/t[i-prefix-1].id() as f64)),
	|(last,stride),next| if (next.id() as f64/last.id() as f64) == stride {(next,stride)} else{(next,0.0)}) != (&t[i+cont-1],0.0));
	
	entropy(&freqs)
	//TODO: turn this into some useful number
	//freqs holds the distribution of elements that are accessed after sequences of regularly multiplied accesses
}


/// Produces a list recording how often a sequence of strides is continued.  Sequences can overlap.
/// 
/// Very slow on traces with very long orderly sequences.
pub fn linear_function_continuation<I: Item>(trace: &Trace<I>) -> Vec<f64>{
	let mut probs = Vec::new();
	let mut max_prefix = 0;
	let mut prefix = 1;
	dbg!("Collecting linear function data...");
	//Checks every length up to the longest that occurs in the trace
	while max_prefix == 0 {
		if prefix % 10 == 0{dbg!(prefix);}
		
		let freqs = trace.stride_histogram(&|t: &Trace<I>, i: usize| i >= prefix && 
		t.strides()[i-prefix..i+1].iter().fold((true,t.strides()[i-prefix]) , |(state,last),&next| (state && last == next,next)).0);
		
		let continued: u32 = freqs.values().sum();
		probs.push(continued as f64);
		if continued == 0{
			max_prefix = prefix;
		}
		
		prefix += 1;
	}
	dbg!("Frequency list done");

	probs
}

/// Creates a vector holding the differences between consecutive items (stride) in `trace`.
pub fn trace_strides<I: Item>(trace: &Vec<I>) -> Vec<i64>{
	let mut strides = Vec::new();
	for i in 1..trace.len(){
		strides.push(trace[i].id() as i64 - trace[i-1].id() as i64);
	}
	strides
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
                trace.stack_distances(false).inner(),
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

        use crate::condition::NoCondition;

        macro_rules! frequency_test {
            ($name:ident: $($in:expr),* => $($out:expr),*) => {
                #[test]
                fn $name() {
                    assert_eq!(Trace::from(vec![$($in),*]).frequency_histogram(&NoCondition::default()), HashMap::from([$($out),*]))
                }
            };
        }

        frequency_test!(basic: 1, 2, 3 => (1, 1), (2, 1), (3, 1));
        frequency_test!(repeated: 1, 1, 1 => (1, 3));
        frequency_test!(one_two: 1, 2, 1, 1, 1 => (1, 4), (2, 1));
        frequency_test!(one_repeated: 1, 2, 3, 1 => (1, 2), (2, 1), (3, 1));
        // frequency_test!(empty: => );
    }

    mod entropy {
        use super::*;

        use crate::condition::NoCondition;

        macro_rules! entropy_test {
            ($name:ident: $($in:expr),* => $out:expr) => {
                #[test]
                fn $name() {
                    assert!((entropy(&Trace::from(vec![$($in),*]).frequency_histogram(&NoCondition::default())) - $out).abs() <= 0.0001)
                }
            };
        }

        entropy_test!(one_item: 0,0,0,0 => 0.0);
        entropy_test!(basic_uniform: 0,1,1,0,1,0 => 1.0);
        entropy_test!(unbalanced: 0,1,2,0,2,0,0,3 => 1.75);
        entropy_test!(precise_value: 0,1,2,0,2,0,0 => 1.37878);
    }
}
