# Generic Data Types

Rust _Generics_ can be used like type templates in _C++_ 

Note that Rust's type inference allows you to omit `T` at
the call site most of the time.

```rust
fn print_max<T>(x: T, y: T) {
    if x >= y {
        println!("{x} >= {y}");
    } else {
        println!("{y} > {x}");
    }
}

fn main() {
    print_max::<u16>(123, 321);
    print_max(3.14, 1.414);
}
```

Use traits to restrict types that can be passed

```rust
fn print_max<T: std::cmp::PartialOrd + std::fmt::Display>(x: T, y: T) {
    if x >= y {
        println!("{x} >= {y}");
    } else {
        println!("{y} > {x}");
    }
}

fn main() {
    print_max(123, 321);
    print_max(3.14, 1.414);
}
```

Rust uses _monomorphization_ to turn generics into specifics at compile time.
The allows Rust to only build the type signatures needed, and to do it early
so the code can be optimized.



