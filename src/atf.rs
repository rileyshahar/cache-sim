//! Utilities for parsing `atf` files.
//!
//! An atf file is a csv. It begins with one _header row_, which is not semantically interpreted by
//! the parser, and can contain any metadata (e.g. units, zeros, source, etc.) relevant to the
//! specific dataset. The header row should begin with a single `#`; any row beginning with a `#`
//! is smiilarly not parsed.
//!
//! The data section contains rows consisting of:
//!
//! - The `identifier`, a unique reference to the _cache item_.
//! - A `timestamp`, in nanoseconds since an arbitrary zero.
//! - Optype, `R` or `W` for read or write.
//! - Size, in arbitrary units.
//! - Any number of cost columns, each representing a different kind of cost of the identifier.

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
enum Operation {
    #[serde(alias = "R")]
    Read,
    #[serde(alias = "W")]
    Write,
}

/// A record of a single operation in a trace.
///
/// This represents a single row of the csv.
#[derive(Debug, Deserialize, PartialEq)]
pub struct OpRecord {
    accessed_item_id: String,
    nanos_since_zero: u32, // TODO: should this be a float
    optype: Operation,
    size: f64,
    cost: Vec<f64>,
}

/// Parse a file-like object into a vector of oprecords.
///
/// # Errors
/// If the csv does not conform to the `atf` standard.
pub fn parse<R: std::io::Read>(input: R) -> Result<Vec<OpRecord>, csv::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(input);

    rdr.deserialize()
        // `Result` implements iterator, so when we collect this it will give us the first error if
        // there are any errors, or else will give us the vector of [`OpRecord`]s.
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() -> Result<(), csv::Error> {
        const DATA: &[u8] = b"# item id, timestamp, operation, bytes, latency (ns)
0,1,R,1,1";

        let out = parse(DATA)?;
        assert_eq!(
            out,
            vec![OpRecord {
                accessed_item_id: "0".to_string(),
                nanos_since_zero: 1,
                optype: Operation::Read,
                size: 1.0,
                cost: vec![1.0],
            }],
        );

        Ok(())
    }

    #[test]
    fn multiline_parser() -> Result<(), csv::Error> {
        const DATA: &[u8] = b"# this is my cool header!
1,2,R,4.5,7,6
0,16,W,3.1,4,2.5
1,4,R,3,2,1.2";

        let out = parse(DATA)?;
        assert_eq!(
            out,
            vec![
                OpRecord {
                    accessed_item_id: "1".to_string(),
                    nanos_since_zero: 2,
                    optype: Operation::Read,
                    size: 4.5,
                    cost: vec![7.0, 6.0],
                },
                OpRecord {
                    accessed_item_id: "0".to_string(),
                    nanos_since_zero: 16,
                    optype: Operation::Write,
                    size: 3.1,
                    cost: vec![4.0, 2.5],
                },
                OpRecord {
                    accessed_item_id: "1".to_string(),
                    nanos_since_zero: 4,
                    optype: Operation::Read,
                    size: 3.0,
                    cost: vec![2.0, 1.2],
                },
            ],
        );

        Ok(())
    }
}
