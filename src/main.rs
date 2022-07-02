use cache_sim::condition::Condition;
use std::collections::HashMap;
use std::fs::File;
use itertools::Itertools;

use cache_sim::{atf::parse, GeneralModelItem, NoCondition, Trace, LastNItems};

fn main() -> anyhow::Result<()> {
    let trace = Trace::from(
        parse(include_bytes!("traces/ycsb-sample.atf").as_slice())?
            .into_iter()
            .map(GeneralModelItem::from)
            .collect::<Vec<_>>(),
    );

    // let stack_distances = trace.stack_distances();

    // to_csv("YCSB Sample", &[], &stack_distances)?;

    let file = File::create("histograms.csv")?;
    let mut conditions: HashMap<String, Box<dyn Condition<GeneralModelItem>>> =
        HashMap::with_capacity(2);

    // TODO: is there a way to statically create a hashmap with type-erased values?
    conditions.insert(String::from("NoCondition"), Box::new(NoCondition));
    conditions.insert(
        String::from("EqualsPrevious"),
        Box::new(|t: &Trace<_>, i| i > 0 && t[i - 1] == t[i]),
    );
    
    for item in trace.iter().unique().copied().collect::<Vec<_>>(){
		let name = format!("After{}",item.to_string());
		conditions.insert(
        name,
        Box::new(LastNItems::new(vec![item])),
    );
	}
	dbg!(conditions.len());

    trace.write_conditional_frequencies(conditions, || Ok(file.try_clone()?))?;

    // let histogram = trace.frequency_histogram(&|t: &Trace<_>, i| i > 0 && t[i-1]==t[i]);
    // let items = trace.iter().unique().copied().collect::<Vec<_>>();
    //
    // histogram_out("NoCondition", &entropy(&histogram), &histogram, &items, file.try_clone()?)?;
    // histogram_out("Test2", &entropy(&histogram), &histogram, &items, file)?;

    Ok(())
}
