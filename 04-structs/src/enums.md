# Enums

Enumerated types can be a group of related constants as in _C_

```rust
#[derive(PartialEq, PartialOrd)]
enum CharOrdinal {
    A,  // implicit discriminator starts at 0
    B,
    C,
    D,
}

enum PrimaryColor {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    if CharOrdinal::C > CharOrdinal::B {
         println!("C (position {}) is after B (position {})",
             CharOrdinal::C as i32, CharOrdinal::B as i32);
    }

    let purple = PrimaryColor::Red as u32 | PrimaryColor::Blue as u32;
    println!("The color purple is {:#x}", purple);
}
```