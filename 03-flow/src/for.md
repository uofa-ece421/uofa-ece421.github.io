# Iterated loops

The "classical" DIY style of iteration:

```rust
fn main() {
    let a = [10.0, 20.0, 30.0, 40.0, 50.0];
    let mut index = 0;
    let mut sum = 0.0;

    while index < 5 {
        sum += a[index];
        index += 1;
    }

    println!("The mean is {:.1}", sum/(index as f32));
}
```

For more on formatting see: [std::fmt](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/fmt/index.html)

### `for` Loops

```rust
fn main() {
    let a = [10.0, 20.0, 30.0, 40.0, 50.0];
    let mut sum = 0.0;

    for element in a {
        sum += element;
    }

    println!("The mean is {:.1}", sum/(a.len() as f32));
}
```

The `for` loop can be applied to collextions that define the `IntoIterator` trait
- the iterator implementation is guaranteed safe
  - avoids overruns and underruns, length mismatches, etc.
- also guaranteed fast
  - avoids the bounds checks that are present in DIY iterated loops




