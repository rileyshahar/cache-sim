use cache_sim::{Cache, Lfu, Trace};

// const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut c = Cache::<Lfu>::new(3);

    // for i in INPUT.lines().map(|n| n.parse().unwrap()) {
    //     l.access(i);
    //     f.access(i);
    //     r.access(i);
    // }

    let trace = Trace::from(vec![1, 0, 2, 2, 3]);
    c.run_trace(&trace);

    dbg!(c.set());
}
