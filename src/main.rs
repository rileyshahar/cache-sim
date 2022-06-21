use std::error::Error;

use cache_sim::{atf::parse, trace::entropy, GeneralModelItem, LastNItems, Trace};

// const INPUT: &str = include_str!("input.txt");

fn main() {
    // let mut c = Cache::<Lfu>::new(3);

    // for i in INPUT.lines().map(|n| n.parse().unwrap()) {
    //     l.access(i);
    //     f.access(i);
    //     r.access(i);
    // }

    // let trace = Trace::from(
    //     parse(include_bytes!("traces/ycsb-sample.atf").as_slice())?
    //         .into_iter()
    //         .map(GeneralModelItem::from)
    //         .collect::<Vec<_>>(),
    // );

    let trace = Trace::from(vec![0, 0, 1, 1, 2]);

    let histogram = trace
        .frequency_histogram(&|t: &Trace<u32>, i| i != 0 && t.inner()[i] == t.inner()[i - 1] + 1);

    for (item, count) in &histogram {
        println!("{},{}", item, count);
    }
    println!("{}", entropy(&histogram));
}
