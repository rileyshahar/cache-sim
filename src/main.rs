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

    dbg!(trace.stack_distances().histogram());

    Ok(())
}

// fn main() {
//     // let mut c = Cache::<Lfu>::new(3);

//     let mut c3 = Cache::<Landlord, (), GeneralModelItem>::new(3);
//     let mut c4 = Cache::<Landlord, (), GeneralModelItem>::new(4);
//     let mut c5 = Cache::<Landlord, (), GeneralModelItem>::new(5);

//     let mut gen = GeneralModelGenerator::default();

//     let a = gen.item(1.0, 1);
//     let b = gen.item(2.0, 2);
//     let c = gen.item(1.0, 2);
//     let d = gen.item(1.0, 1);

//     let trace = [a, b, c, a, d].into_iter().collect();

//     c3.run_trace(&trace);
//     dbg!(c3.set());

//     c4.run_trace(&trace);
//     dbg!(c4.set());

//     c5.run_trace(&trace);
//     dbg!(c5.set());
// }
