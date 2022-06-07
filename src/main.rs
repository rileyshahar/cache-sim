use cache_sim::{stats, Cache, Fifo, Lru, Rand};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut l = Cache::<Lru, (stats::HitCount, stats::MissCount)>::new(3);
    let mut f = Cache::<Fifo, (stats::HitCount, stats::MissCount)>::new(3);
    let mut r = Cache::<Rand, (stats::HitCount, stats::MissCount)>::new(3);

    for i in INPUT.lines().map(|n| n.parse().unwrap()) {
        l.access(i);
        f.access(i);
        r.access(i);
    }

    // let trace = &l.stat().2;

    // let (distances, infinities) = StackDistance::compute(trace).histogram();

    // for i in distances {
    //     println!("{}, ", i);
    // }

    // println!("infinities: {}", infinities);

    dbg!(l.stat());
    dbg!(f.stat());
    dbg!(r.stat());
}
