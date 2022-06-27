//! Utilities for outputting data.

use serde::{ser::SerializeSeq, Serialize};

use crate::trace::StackDistance;

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
        let mut seq = serializer.serialize_seq(Some(self.stack_distances.len()))?;

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
