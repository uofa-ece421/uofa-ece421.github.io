# Iterators

An iterator is something that has a `next()` and returns `None` when finished

```rust
fn main() {
    // "0..3" is an Iterator that generates: 0, 1, and 2
    let mut sequence = 0..3;

    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
}
```

A `for` loop uses `.into_iter()` and exits when finished

```rust
fn main() {
    let a = [1, 2, 3, 4];
    let a_iter = a.iter();
    for i in a_iter {
        println!("for {0}", i);
    }
}
```

There is a rich set of operations defined for iterators in [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)


```rust
fn main() {
    for i in (0..=3).rev() {
        print!("{i} ");
    }
    println!("Blast-off!");

    for i in (0..).skip(10).take(4) {
        print!("{} ", i);
    }
}
```
