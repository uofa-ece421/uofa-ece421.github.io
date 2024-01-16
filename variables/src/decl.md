# Declaring Variables

Variables are declared using the `let` keyword:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {}", x);
}
```

Default immutability is for your safety.
In C this is legal (and common):
```c
if (a = b) {
```
and fine as long as you didn't intend to write:
```c
if (a == b) {
```

<u>Best practice</u>: initialize at declaration time

```rust
fn main() {
    // constants must be computable at compile time
    const POUND_TO_OUNCE : u32 = 16;

    let mut oz = 1;
    oz = oz * POUND_TO_OUNCE;

    let lb = oz;
    let lb = oz/POUND_TO_OUNCE;

    let everything = "everything";
    let everything = everything.len();
}
```

Note: there is one instance of `oz` and two instances of `lb`
- `oz` is mutable, `lb` is not
