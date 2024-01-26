# Move vs Copy

Simple primitive types have known, fixed size, and can be quickly copied

```rust
fn main() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}
```

Dynamically sized object are somewhat more involved

```rust
fn main() {
    let x = String::from("hello");
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

Here's how a `String` is represented:

<img alt="String representation" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

Rust uses a "shallow" copy of `x` to `y` (i.e. just the `String` struct).
We say the ownership of the heap data is _Moved_ to `y`

<img alt="String move" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

Rust will not _automatically_ create "deep" copies of heap data.
However, you can ask for a deep copy with the `clone()` method

<img alt="String copy" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

```rust
fn main() {
    let x = String::from("hello");
    let y = x.clone();

    println!("x = {}, y = {}", x, y);
}
```

