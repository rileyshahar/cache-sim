use cache_sim::condition::Condition;
use std::collections::HashMap;
use std::fs::File;
use std::env;
use itertools::Itertools;

use cache_sim::{atf::parse, output::to_csv, GeneralModelItem, NoCondition, Trace, LastNItems, trace::entropy};

fn main() -> anyhow::Result<()> {
	let args: Vec<String> = env::args().collect();
	let atf_name = &format!("{}.atf",args[1]);
	
	let trace_file = File::open(&format!("src/traces/{}",atf_name))?;
	
    let trace = Trace::from(
        parse(trace_file)?
            .into_iter()
            .map(GeneralModelItem::from)
            .collect::<Vec<_>>(),
    );
    
	dbg!("parsed");
	let record_file = File::options().append(true).create(true).open("src/histograms/stack-distances.csv")?;
	dbg!("file open");
    let stack_distances = trace.stack_distances();
    //let stack_distances = Trace::from(vec![0,0]).stack_distances();
    dbg!("stack dists done");
	
    to_csv(&args[1], &[trace.average_entropy(),entropy(&trace.frequency_histogram(&NoCondition))], &stack_distances, record_file)?;
	dbg!("printed stack distances");
	
	
	// Output frequency histograms
    let file = File::create(&format!("src/histograms/{}-histograms.csv",&args[1]))?;
    let mut conditions: HashMap<String, Box<dyn Condition<GeneralModelItem>>> =
        HashMap::with_capacity(2);

    // TODO: is there a way to statically create a hashmap with type-erased values?
    conditions.insert(String::from("NoCondition"), Box::new(NoCondition));
    conditions.insert(
        String::from("EqualsPrevious"),
        Box::new(|t: &Trace<_>, i| i > 0 && t[i - 1] == t[i]),
    );
    
    for item in trace.iter().unique().copied().collect::<Vec<_>>(){
		let name = format!("After{}",item);
		conditions.insert(
        name,
        Box::new(LastNItems::new(vec![item])),
    );
	}
	dbg!("assembled conditions");
    trace.write_conditional_frequencies(conditions, || Ok(file.try_clone()?))?;
	
	
    Ok(())
}
