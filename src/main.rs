use std::error::Error;

use cache_sim::{atf::parse, GeneralModelItem, Trace};

// const INPUT: &str = include_str!("input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    // let mut c = Cache::<Lfu>::new(3);

    // for i in INPUT.lines().map(|n| n.parse().unwrap()) {
    //     l.access(i);
    //     f.access(i);
    //     r.access(i);
    // }

    let trace = Trace::from(
        parse(include_bytes!("traces/ycsb-sample.atf").as_slice())?
            .into_iter()
            .map(GeneralModelItem::from)
            .collect::<Vec<_>>(),
    );

    dbg!(trace.stack_distances().inner());

    Ok(())
}
