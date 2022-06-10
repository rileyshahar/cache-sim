use cache_sim::{stats, Cache, Opt, Trace};

// const INPUT: &str = include_str!("input.txt");

fn main() {
	let trace = Trace::from(vec![0, 1, 2, 2, 3, 2, 4, 1, 3, 4, 1, 0, 1, 3, 2]);
    let mut c = Cache::<Opt,(stats::HitCount,stats::MissCount)>::with_replacement_policy(Opt::on_trace(&trace),3);

    // for i in INPUT.lines().map(|n| n.parse().unwrap()) {
    //     l.access(i);
    //     f.access(i);
    //     r.access(i);
    // }

    c.run_trace(&trace);

    dbg!(c.set());
    dbg!(c.stat());
}
