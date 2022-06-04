use cache_sim::{Cache, HitRate, Item, Lru, StackDistance, Trace, TraceStat};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut c = Cache::new(Lru::new(), 10);
    c.track(HitRate::default());

    for i in INPUT.lines().map(|n| n.parse().unwrap()) {
        c.access(Item(i));
    }

    let trace: Trace = INPUT
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
        .into();

    let (distances, infinities) = StackDistance::compute(trace).histogram();

    for i in distances {
        println!("{}, ", i);
    }

    println!("infinities: {}", infinities);

    // for i in [0, 1, 2, 3, 0, 0, 0, 1, 2, 1] {
    //     c.access(Item(i));
    //     println!("{}", c);
    // }

    dbg!(c.statistics());
}
