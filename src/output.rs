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
            serializer.serialize_seq(Some(2 + self.stats.len() + self.stack_distances.iter().filter(|&&i| i != 0).count()))?;

        seq.serialize_element(self.name)?;
        for stat in self.stats {
			if stat.fract() == 0.0 {
				seq.serialize_element(&format!("{:.0}",stat))?;
			}
			else{
            	seq.serialize_element(&format!("{:.5}",stat))?;
            }
        }

        seq.serialize_element(&self.infinities)?;

        for (distance, num) in self.stack_distances.iter().enumerate().filter(|&(_,&val)| val != 0) {
            seq.serialize_element(&format!("{}:{}",distance,num))?;
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
}

impl<I: Item, H: std::hash::BuildHasher> Serialize for FreqHistRow<'_, I, H> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2 + self.histogram.len()))?;

        seq.serialize_element(self.name)?;
        seq.serialize_element(&format!("{:.5}",self.entropy))?;

        for (item,freq) in self.histogram {
        	seq.serialize_element(&format!("{}:{}",item,freq))?;
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
    writer: W,
) -> Result<(), csv::Error> {
    let output = FreqHistRow {
        name,
        entropy,
        histogram,
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

struct LinearContRow<'a> {
    // TODO: does this need to be owned
    name: &'a str,
    length: usize,
    probs: &'a [f64],
}

impl Serialize for LinearContRow<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2 + self.probs.len()))?;

        seq.serialize_element(self.name)?;
        
        seq.serialize_element(&self.length)?;

        for prob in self.probs {
			if prob.fract() == 0.0 {
				seq.serialize_element(&format!("{:.0}",prob))?;
			}
			else{
            	seq.serialize_element(&format!("{}",prob))?;
            }
        }
        seq.end()
    }
}

pub fn linear_cont_out<W: Write>(
    name: &str,
    length: usize,
    probs: &[f64],
    writer: W,
) -> Result<(), csv::Error> {
    let output = LinearContRow {
        name,
        length,
        probs,
    };

    let mut wtr = csv::Writer::from_writer(writer);

    wtr.serialize(output)
}
