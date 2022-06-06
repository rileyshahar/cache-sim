//! An abstracted cacheable item.

/// Abstracts over a single item in a cache.
///
/// In the future, this will probably include things like size, cost, etc., for simulating more
/// complex caching models.
pub trait Item:
    Default + std::fmt::Debug + std::fmt::Display + PartialEq + Eq + Copy + Clone + std::hash::Hash
{
}

impl Item for u32 {}
