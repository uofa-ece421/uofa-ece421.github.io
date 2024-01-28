# Type aliases

A new name for an existing type.

```rust
type Point = (i32, i32);

use Point as Coordinate;

fn main() {
    let p: Point = (100, 100);
    let northeast: Coordinate = p;

    println!("wind direction today: {:?}", northeast);
}
```

Type aliases are _synonyms_ of their underlying type - they are not a new type.
