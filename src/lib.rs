#![doc = include_str!("../README.md")]

pub mod atf;
mod cache;
pub mod condition;
pub mod item;
pub mod output;
pub mod replacement_policy;
pub mod stats;
pub mod trace;

pub use cache::Cache;
pub use condition::{LastNItems, NoCondition};
pub use item::{GeneralModelGenerator, GeneralModelItem};
pub use trace::Trace;

pub use replacement_policy::{Fifo, Landlord, Lfu, Lru, Mru, Rand};
