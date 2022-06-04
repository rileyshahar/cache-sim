use cache_sim::{Cache, Item, Lru};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut c = Cache::new(Lru::new(), 3);

    for i in INPUT.lines().map(|n| n.parse().unwrap()) {
        c.access(Item(i));
        println!("{}", c);
    }

    // for i in [0, 1, 2, 3, 0, 0, 0, 1, 2, 1] {
    //     c.access(Item(i));
    //     println!("{}", c);
    // }
}
