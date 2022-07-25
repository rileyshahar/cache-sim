use cache_sim::condition::Condition;
use std::collections::HashMap;
use std::fs::File;
use std::env;

use cache_sim::{atf::parse, output::to_csv, output::linear_cont_out, GeneralModelItem, NoCondition, Trace, trace::entropy, trace::linear_function_entropy, trace::exp_function_entropy, trace::linear_function_continuation};

//Call with: cargo run [filename, no extension (assumes .atf)] [prefix] [(optional) use only continued streaks] [(optional) print stack distances Y/N] [(optional) print histograms Y/N] [(optonal) print linear continuation data] [(optional) Ignore sizes Y/N]
//TODO: make the optional flags more convenient to use
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
	let mut stack_distances = Trace::<u32>::from(vec![]).stack_distances(true);
	if args.len() > 4 && args[4] == "Y" {
		let mut paging_model = false;
		if args.len() > 7 && args[7] == "N" {
			paging_model = true;
		}
		stack_distances = trace.stack_distances(paging_model);
	}
    dbg!("stack dists done");
    let mut continuation = 0;
	if args.len() > 3 && args[3] == "T"{
		continuation = 1;
	}
	
    to_csv(&args[1],
    &[trace.len() as f64,args[2].parse()?,
    entropy(&trace.frequency_histogram(&NoCondition)),entropy(&trace.stride_histogram(&NoCondition)),
    trace.average_entropy(args[2].parse()?),trace.stride_entropy(args[2].parse()?),
    exp_function_entropy(&trace,args[2].parse()?,continuation),linear_function_entropy(&trace,args[2].parse()?,continuation)],
    &stack_distances, record_file)?;
	//csv header: Name,Trace length,Prefix,Item entropy,Stride entropy,Item conditional entropy,Stride conditional entropy,Exponential function entropy,Linear Function entropy,Infinities,Stack distances
	
	dbg!("printed csv");
	if args.len() > 6 && args[6] == "Y"{
		let linear_file = File::options().append(true).create(true).open("src/histograms/linear_function_data.csv")?;
		linear_cont_out(&args[1],trace.len(),&linear_function_continuation(&trace),linear_file)?;
		dbg!("printed linear continuation data");
	}
	
	if args.len() > 5 && args[5] == "Y"{
		// Output frequency histograms
	    let file = File::create(&format!("src/histograms/{}-histograms.csv",&args[1]))?;
	    let mut conditions: HashMap<String, (Box<dyn Condition<GeneralModelItem>>,bool)> =
	        HashMap::with_capacity(2);
	
	    // TODO: is there a way to statically create a hashmap with type-erased values?
	    conditions.insert(String::from("Items"), (Box::new(NoCondition),false));
	    conditions.insert(String::from("Strides"), (Box::new(NoCondition),true));
	    
	    conditions.insert(String::from("Stride Streaks"), (Box::new(|t: &Trace<_>, i: usize| i > 0 && t.strides()[i] == t.strides()[i-1]),true));//strides that continue a streak
	    
	    //TODO: add more conditions, but not too many
	    
		dbg!("assembled conditions");
	    trace.write_conditional_frequencies(conditions, || Ok(file.try_clone()?))?;
	    dbg!("printed histograms");
    }
	
	
    Ok(())
}
