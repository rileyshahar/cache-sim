A cache simulator.

# Basic Usage

```rust
use std::collections::HashSet;

use cache_sim::{Cache, Lru};

let mut c = Cache::<Lru>::new(3);

c.access(0);
c.access(1);
c.access(2);
c.access(0);
c.access(3);

assert_eq!(c.set(), &HashSet::from([0, 2, 3]));
```

# Configuring the Cache

## Items

Currently, the cache abstractly caches `u32`s, each of which should be read to
represent a different cacheable item, e.x. a block from memory. The cache will
work with any type which implements the `Item` marker trait. For
forwards-compatibliity, this should not be done frivolously, because in the
future this trait will represent properties of more abstract caching models,
like the cost and size of the item.

## Capacity

The `Cache::new` function takes a capacity as a parameter; this allows you to
experiment on arbitrarily-sized caches.

## Replacement Policies

The library contains implementations of several replacement policies,
represented by the first generic type of `Cache`:

```rust
use std::collections::HashSet;

use cache_sim::{Cache, Fifo};

let mut c = Cache::<Fifo>::new(3);

c.access(0);
c.access(1);
c.access(2);
c.access(0);
c.access(3);

assert_eq!(c.set(), &HashSet::from([1, 2, 3]));
```

# Statistics

You can attach statistics to the cache using its second generic type (default
`()`), like so:

```rust
use cache_sim::{Cache, Lru};
use cache_sim::stats::HitCount;

let mut c = Cache::<Lru, HitCount>::new(3);
c.access(0); // miss
c.access(1); // miss
c.access(2); // miss
c.access(0); // hit
c.access(3); // miss
c.access(0); // hit

assert_eq!(c.stat().0, 2);
```

You can track multiple statistics using a tuple:

```rust
use cache_sim::{Cache, Lru};
use cache_sim::stats::{HitCount, MissCount};

let mut c = Cache::<Lru, (HitCount, MissCount)>::new(3);
c.access(0); // miss
c.access(1); // miss
c.access(2); // miss
c.access(0); // hit
c.access(3); // miss
c.access(0); // hit

assert_eq!(c.stat().0.0, 2);
assert_eq!(c.stat().1.0, 4);
```

# Traces

There are also tools available for analyzing abstracted traces, like so:

```rust
use std::collections::HashMap;

use cache_sim::Trace;

let trace = Trace::from(vec![0, 0, 1, 0, 3, 1]);
let frequencies = trace.frequency_histogram();
assert_eq!(frequencies.get(&0), Some(&3));
```
