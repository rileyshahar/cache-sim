use cache_sim::{trace::entropy, Trace, NoCondition};

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

    let trace = Trace::from(vec![0, 1, 2, 0, 2, 0, 0, 3]);

    let histogram = trace.frequency_histogram(&NoCondition);

    for (item, count) in &histogram {
        println!("{},{}", item, count);
    }
    println!("{}", entropy(&histogram));
}
