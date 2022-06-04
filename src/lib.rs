mod cache;
mod replacement_policy;
mod stats;
mod trace;

pub use cache::Cache;
pub use replacement_policy::{Fifo, Lru};
pub use stats::HitRate;
pub use trace::{StackDistance, Trace, TraceStat};

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub struct Item(pub u32); // TODO: figure out what this should be represented as
