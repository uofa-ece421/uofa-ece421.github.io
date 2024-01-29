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

Note that all string constants are of type `&'static str`

Because a slice is a borrowed reference, the data race and reference rules apply

```rust
fn binary_search(needle: i32, haystack: &[i32]) -> bool {
    match haystack.len() {
        0 => false,
        1 => needle == haystack[0],
        len => if needle >= haystack[len/2] {
            binary_search(needle, &haystack[len/2..])
        } else {
            binary_search( needle, &haystack[..len/2])
        },
    }
}

fn main() {
     let pile = [-14, -9, -8, -3, 0, 2, 3, 6, 7, 11, 14];

     let found_minus3 = binary_search(-3, &pile);
     println!("pile contains -3: {}", found_minus3);

     let found_positive = binary_search(3, &pile[4..]);
     println!("positive pile contains 3: {}", found_positive);
}
```
