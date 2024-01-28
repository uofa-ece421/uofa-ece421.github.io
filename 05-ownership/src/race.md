# Race Conditions

A _data race_ is similar to a race condition:
* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.

Rust prevents data races by enforcing:
* any number of immutable references
* or exactly one mutable reference

So this is legal:

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;  // two immutable references

    println!("{}, {}", r1, r2);
}
```

but not

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    println!("{}, {}", s, r2);
}
```

or

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s; 
    let r2 = &mut s; // two mutable references

    println!("{}, {}", r1, r2);
}
```

Dangling pointers are also called "use-after-free"

```rust
fn main() {
    let use_after_free = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```