mod cache;
pub mod replacement_policy;
pub mod stats;
pub mod trace;

pub use cache::Cache;
pub use trace::{StackDistance, Trace};

/// A single item in a cache.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub struct Item(pub u32); // TODO: figure out what this should be represented as
