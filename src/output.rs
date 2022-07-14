//! Utilities for outputting data.

use serde::{ser::SerializeSeq, Serialize};
use std::collections::HashMap;
use std::io::Write;

use crate::item::Item;
use crate::trace::StackDistance;

struct OutputCsvRow<'a> {
    name: &'a str,
    stats: &'a [f64],
    stack_distances: &'a [usize],
    infinities: usize,
}

impl Serialize for OutputCsvRow<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq =
            serializer.serialize_seq(Some(2 + self.stats.len() + self.stack_distances.len()))?;

        seq.serialize_element(self.name)?;
        for stat in self.stats {
            seq.serialize_element(&format!("{:.5}",stat))?;
        }

        seq.serialize_element(&self.infinities)?;

        for distance in self.stack_distances {
            seq.serialize_element(distance)?;
        }
        seq.end()
    }
}

/// Write a set of statistics to a csv on stdout.
///
/// TODO: let us use generic outputs (e.x. to write to file)
///
/// # Errors
/// If writing fails, for example if the output buffer is closed by the OS.
pub fn to_csv<W: Write>(
    name: &str,
    stats: &[f64],
    stack_distances: &StackDistance,
    writer: W,
) -> Result<(), csv::Error> {
    let (stack_distances, infinities) = stack_distances.histogram();
    let output = OutputCsvRow {
        name,
        stats,
        stack_distances: &stack_distances,
        infinities,
    };

    let mut wtr = csv::Writer::from_writer(writer);

    wtr.serialize(output)
}

struct FreqHistRow<'a, I: Item, H: std::hash::BuildHasher> {
    // TODO: does this need to be owned
    name: &'a str,
    entropy: f64,
    histogram: &'a HashMap<I, u32, H>,
    items: &'a [I],
}

impl<I: Item, H: std::hash::BuildHasher> Serialize for FreqHistRow<'_, I, H> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2 + self.items.len()))?;

        seq.serialize_element(self.name)?;
        seq.serialize_element(&format!("{:.5}",self.entropy))?;

        for item in self.items {
            if let Some(freq) = self.histogram.get(item) {
                seq.serialize_element(freq)?;
            } else {
                seq.serialize_element(&0_usize)?;
            }
        }
        seq.end()
    }
}

/// Write a histogram to a row of a csv file.
///
/// The order of the `items` slice determines the order in which frequencies will be written to the
/// csv.
///
/// # Errors
/// If the writing fails.
pub fn histogram_out<I: Item, W: Write, H: std::hash::BuildHasher>(
    name: &str,
    entropy: f64,
    histogram: &HashMap<I, u32, H>,
    items: &[I],
    writer: W,
) -> Result<(), csv::Error> {
    let output = FreqHistRow {
        name,
        entropy,
        histogram,
        items,
    };

    let mut wtr = csv::Writer::from_writer(writer);

    wtr.serialize(output)
}

struct HeaderRow<'a> {
    labels: &'a [String],
}

impl Serialize for HeaderRow<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.labels.len()))?;

        for label in self.labels {
			seq.serialize_element(label)?;
        }
        seq.end()
    }
}

pub fn write_header<W: Write>(
    labels: &[String],
    writer: W,
) -> Result<(), csv::Error> {
    let output = HeaderRow {
        labels,
    };

    let mut wtr = csv::Writer::from_writer(writer);

    wtr.serialize(output)
}
