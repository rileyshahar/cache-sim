#![doc = include_str!("../README.md")]

pub mod atf;
mod cache;
pub mod item;
pub mod replacement_policy;
pub mod stats;
pub mod trace;
pub mod condition;

pub use cache::Cache;
pub use item::{GeneralModelGenerator, GeneralModelItem};
pub use trace::Trace;
pub use condition::{NoCondition, LastNItems};

pub use replacement_policy::{Fifo, Landlord, Lfu, Lru, Mru, Rand};
