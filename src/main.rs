use cache_sim::{Cache, Item, Lru};

fn main() {
    let mut c = Cache::new(Lru {});

    c.access(Item(0));
    println!("{}", c);

    c.access(Item(1));
    println!("{}", c);

    c.access(Item(0));
    println!("{}", c);
}
