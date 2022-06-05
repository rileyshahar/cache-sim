mod cache;
mod replacement_policy;
pub mod stats;
pub mod trace;

pub use cache::Cache;
pub use replacement_policy::{Fifo, Lru};
pub use trace::{StackDistance, Trace};

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub struct Item(pub u32); // TODO: figure out what this should be represented as
