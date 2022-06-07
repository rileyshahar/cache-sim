#![doc = include_str!("../README.md")]

mod cache;
pub mod item;
pub mod replacement_policy;
pub mod stats;
pub mod trace;

pub use cache::Cache;
pub use trace::Trace;

pub use replacement_policy::{Fifo, Lfu, Lru, Mru, Rand};
