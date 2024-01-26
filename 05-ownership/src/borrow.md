# References and Borrowing

Moving ownership all the time is cumbersome:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

Rust allows functions to _borrow_ a value using "references"

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Just like _C_, the operator `&` means "address-of".
References are also immutable by default

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("The length of '{}' is {}.", s, s.len());
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
