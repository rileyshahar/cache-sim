use std::error::Error;
use std::fs::File;
use itertools::Itertools;

use cache_sim::{atf::parse, output::to_csv, output::histogram_out, trace::entropy, GeneralModelItem, Trace, NoCondition};

fn main() -> Result<(), Box<dyn Error>> {
    let trace = Trace::from(
        parse(include_bytes!("traces/ycsb-sample.atf").as_slice())?
            .into_iter()
            .map(GeneralModelItem::from)
            .collect::<Vec<_>>(),
    );

    let stack_distances = trace.stack_distances();

    to_csv("YCSB Sample", &[], &stack_distances)?;
   	
   	let histogram = trace.frequency_histogram(&|t: &Trace<_>, i| i > 0 && t[i-1]==t[i]);
   	let items = trace.iter().unique().copied().collect::<Vec<_>>();
   	let file = File::create("test.csv")?;
    
    histogram_out("NoCondition", &entropy(&histogram), &histogram, &items, file.try_clone()?)?;
    histogram_out("Test2", &entropy(&histogram), &histogram, &items, file)?;
    

    Ok(())
}
