use cache_sim::{Cache, Fifo, HitRate, Item, Lru};

// const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut l = Cache::<Lru, HitRate>::new(3);
    let mut f = Cache::<Fifo, HitRate>::new(3);

    // for i in INPUT.lines().map(|n| n.parse().unwrap()) {
    //     l.access(Item(i));
    //     f.access(Item(i));
    // }

    // let trace: Trace = INPUT
    //     .lines()
    //     .map(|n| n.parse().unwrap())
    //     .collect::<Vec<_>>()
    //     .into();

    // let (distances, infinities) = StackDistance::compute(trace).histogram();

    // for i in distances {
    //     println!("{}, ", i);
    // }

    // println!("infinities: {}", infinities);

    for i in [0, 1, 2, 0, 3, 0] {
        l.access(Item(i));
        println!("L: {}", l);

        f.access(Item(i));
        println!("F: {}", f);
    }

    dbg!(l.stat());
    dbg!(f.stat());
}
