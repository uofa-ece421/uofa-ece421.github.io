# Slices

A _String slice_ is a fixed size reference to `String` data.

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..];

    println!("{}, {}!", hello, world);
}
```

The object `s` is type `String`; the slice `world` is type `&str`

<img src="img/trpl04-06.svg" alt="Slice representation" class="center" style="width: 50%;" />

Because a slice is a borrowed reference, the data race and reference rules apply



