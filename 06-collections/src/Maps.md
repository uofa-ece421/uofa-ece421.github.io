# Maps

The basic Rust _hash map_ has type `HashMap<K, V>`, which stores a mapping from keys of type `K` to values of type `V`.

```rust
use std::collections::HashMap;

fn main() {
    let mut standings = HashMap::new();

    standings.insert(String::from("Oilers"), 60);
    standings.insert(String::from("Flames"), 50);

    let my_team = String::from("Oilers");

    let points = standings.get(&my_team).copied().unwrap_or(0);

    println!("{} have {} points", my_team, points);
}
```

`get()` returns `Option<&V>` in case the key isn't found.
`copied()` is used to get `Option<i32>` instead of `Option<&i32>`.

Simple types are copied into the map, but owned types are _moved_ in and consumed, which means they can no longer be accessed after they have been inserted.

If you update a value in the map, the ownership rules move the new value in and drop the existing value.

You can also check that a key exists before inserting a value

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}





