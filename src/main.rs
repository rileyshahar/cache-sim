use std::error::Error;

use cache_sim::{atf::parse, output::to_csv, GeneralModelItem, Trace};

// const INPUT: &str = include_str!("input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let trace = Trace::from(
        parse(include_bytes!("traces/ycsb-sample.atf").as_slice())?
            .into_iter()
            .map(GeneralModelItem::from)
            .collect::<Vec<_>>(),
    );

    let stack_distances = trace.stack_distances();

    to_csv("YCSB Sample", &[], &stack_distances)?;

    Ok(())
}
