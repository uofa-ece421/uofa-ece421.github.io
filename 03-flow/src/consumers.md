# Using Iterators

<u>Consuming adaptors</u> call `next`, which _consumes_ the iterator by using it up

```rust
fn main() {
    let a = [10.0, 20.0, 30.0, 40.0, 50.0];

    let mean = a.iter().sum::<f32>()/(a.len() as f32);
    println!("idiomatic average: {}", mean);
}
```

<u>Iterator adaptors</u> take an iterator and produce a new one

```rust
fn main() {
    let a = ["1.0", "two", "NaN", "4.0"];

    let good: Vec<f32> = a.iter()
                          .map(|s| s.parse())
                          .filter(|s| s.is_ok())
                          .map(|result| result.unwrap())
                          .collect();
    println!("parseable: {:?}", good);
}
```

The `|x|` is called a _closure_. Closures are anonymous functions with a number
of interesting properties, but for now we are using them to give a name to the
incoming element.

The implementors of Rust have worked very hard to make iterators fast
- another example of the _zero-overhead_ principle at work


