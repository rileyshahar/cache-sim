//! Utilities for outputting data.

use serde::{ser::SerializeSeq, Serialize};
use std::collections::HashMap;
use std::io::Write;



use crate::trace::StackDistance;
use crate::item::Item;

struct OutputCsvRow<'a> {
    // TODO: does this need to be owned
    name: &'a str,
    stats: &'a [u32],
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
            seq.serialize_element(stat)?;
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
pub fn to_csv(
    name: &str,
    stats: &[u32],
    stack_distances: &StackDistance,
) -> Result<(), csv::Error> {
    let (stack_distances, infinities) = stack_distances.histogram();
    let output = OutputCsvRow {
        name,
        stats,
        stack_distances: &stack_distances,
        infinities,
    };

    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    wtr.serialize(output)
}

struct FreqHistRow<'a> {
    // TODO: does this need to be owned
    name: &'a str,
    entropy: &'a f64,
    frequencies: &'a [u32],
}

impl Serialize for FreqHistRow<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq =
            serializer.serialize_seq(Some(2 + self.frequencies.len()))?;

        seq.serialize_element(self.name)?;
        seq.serialize_element(&self.entropy)?;
        
        for freq in self.frequencies {
            seq.serialize_element(freq)?;
        }
        seq.end()
    }
}

pub fn histogram_out<I: Item, W: Write>(
    name: &str,
    entropy: &f64,
    histogram: &HashMap<I, u32>,
    items: &Vec<I>,
    writer: W,
) -> Result<(), csv::Error> {
	let mut frequencies = Vec::<u32>::default();
	for item in items{
		if let Some(freq) = histogram.get(item){
			frequencies.push(*freq);
		}
		else{
			frequencies.push(0);
		}
	}
    let output = FreqHistRow {
        name,
        entropy,
        frequencies: &frequencies[..],
    };

    let mut wtr = csv::Writer::from_writer(writer);

    wtr.serialize(output)
}
