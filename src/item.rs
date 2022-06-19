//! An abstracted cacheable item.

/// Abstracts over a single item in a cache.
pub trait Item:
    Default + std::fmt::Debug + std::fmt::Display + PartialEq + Eq + Copy + Clone + std::hash::Hash
{
    /// The cost to cache the item; i.e. the cost of a miss.
    fn cost(&self) -> f64;

    // TODO: should this be a float? the young paper says that sizes are integral, so we're making
    // them integers for now because floats work strangely, but maybe to be more general we should
    // allow floats
    /// The size of the item in the cache.
    fn size(&self) -> u32;
}

impl Item for u32 {
    fn cost(&self) -> f64 {
        1.0
    }
    fn size(&self) -> u32 {
        1
    }
}

/// A cacheable item with arbitrary const cost and size.
///
/// We implement Hash and Eq by hand to allow floating point costs and sizes. They are simple,
/// naive wrappers around the implementations for the u32 identifier. This means that you should
/// _make sure_ that the identifier is different for each item in your trace, or else the trace
/// will not work correctly.
#[allow(clippy::module_name_repetitions)]
#[derive(Default, Debug, Copy, Clone)]
pub struct GeneralModelItem {
    uid: u32,
    cost: f64,
    size: u32,
}

impl GeneralModelItem {
    /// Create a new general model item.
    ///
    /// If you don't care about the unique identifier, prefer using a [`GeneralModelGenerator`].
    #[must_use]
    pub const fn new(uid: u32, cost: f64, size: u32) -> Self {
        Self { uid, cost, size }
    }
}

impl std::hash::Hash for GeneralModelItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}

impl std::cmp::PartialEq for GeneralModelItem {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl std::cmp::Eq for GeneralModelItem {}

impl std::fmt::Display for GeneralModelItem {
	
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		/*
        write!(
            f,
            "{}: Size = {}, Cost = {}",
            self.uid, self.size, self.cost
        )
        */
        write!(f,"{}", self.uid)
    }
}

impl Item for GeneralModelItem {
    fn cost(&self) -> f64 {
        self.cost
    }

    fn size(&self) -> u32 {
        self.size
    }
}

/// A generator for general model items.
///
/// If you don't care about the unique identifier, this is the preferred way to create these items.
#[derive(Default)]
pub struct GeneralModelGenerator {
    counter: u32,
}

impl GeneralModelGenerator {
    pub fn item(&mut self, cost: f64, size: u32) -> GeneralModelItem {
        let ret = GeneralModelItem {
            uid: self.counter,
            cost,
            size,
        };
        self.counter += 1;
        ret
    }

    /// Create a new general model item generator.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
