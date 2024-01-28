# Option

Rust does not have _null_. _Null_ is a value that means there is no value,
very much like the empty set. Instead, Rust uses the _Option_ enum to encode
the concept of present or absent.

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

_Option_ is pervasive in Rust. It has *lots* of helper [functions](https://doc.rust-lang.org/std/option/enum.Option.html)

```rust
// The Rust prelude automatically "uses" std::Option::{Some, None}
fn main() {
    let present_u32 = Some(5);
    let absent_u32: Option<u32> = None;

    assert_eq!(present_u32.is_some(), true);
    assert_eq!(absent_u32.is_none(), true);

    assert_eq!(present_u32.is_some_and(|val| val == 5), true);

    if let Some(val) = present_u32 {
        println!("present {val}");
    }

    match absent_u32 {
        Some(val) => println!("absent {val}"),
        None => println!("absent is absent"),
    }

    println!("unwrapped {}", absent_u32.unwrap_or_default());
}
```

`match` is very powerful and also pervasive. The power comes from the
expressiveness and variety of the patterns, and because Rust ensures that all
possible case are handled.
