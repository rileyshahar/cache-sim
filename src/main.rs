use cache_sim::{Cache, Item, Lru};

fn main() {
    let mut c = Cache::new(Lru::new(), 3);

    c.access(Item(0));
    println!("{}", c);

    c.access(Item(1));
    println!("{}", c);

    c.access(Item(0));
    println!("{}", c);

    c.access(Item(2));
    println!("{}", c);

    c.access(Item(3));
    println!("{}", c);
}
