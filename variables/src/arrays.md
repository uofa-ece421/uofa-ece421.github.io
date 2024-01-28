# Primitve Compound Types

Compound types can group multiple values into one type.
Once declared, these primitive compound types have fixed size.

### Tuples

A comma separated list of primitive types:

```rust
fn main() {
    let tup: (i32, f64, u8) = (100, 100.0, 100);

    // This is called destructuring
    let (i, f, b) = tup;

    println!("The float value is {} = {}", f, tup.1);
}
```

### Arrays

A comma separated list of the _same_ type:

```rust
fn main() {
    let a = [1, 2, 3, 4];
    let specific: [i16; 4] = a;
    // initialize all elements to the same value
    let repeat = [0u8; 4];

    println!("index as expected: {}", specific[0]);
    println!("dbg pretty print: {:?}", repeat);
}

    