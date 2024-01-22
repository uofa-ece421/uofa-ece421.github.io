# Structs

```rust
struct Rectangle {
    width: u32,
    depth: u32,
}

fn main() {
    let square = Rectangle {
        width: 16,
        depth: 16,
    };

    println!("dimension: {}x{}", square.width, square.depth);
}
```

Unlike the `type` keyword, `struct` actually defines a new type

```rust
#[derive(Debug)]
struct Point(f64, f64); 

type Complex = (f64, f64);

fn main() {
    let origin = Point(0.0, 0.0);
    let imaginary: Complex = (0.0, 1.0);

    println!("struct: {:#?}", origin);
    println!("type: {:#?}", imaginary);
}
```





    
