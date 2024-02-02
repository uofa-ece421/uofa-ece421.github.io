# Lifetimes

Lifetimes are a type of generic used to ensure references are valid.

Remember that every value has an owner, and when the owner goes out of scope
the value is dropped. Every reference also has a _lifetime_, which must be
within the lifetime of the value it points at.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

Rust associates every scope with a lifetime, e.g. `'a` and `'b` above.
Most of the time, the borrow checker can ensure there are no dangling
references, so you don't have to specify the lifetimes.

```rust
#[derive(Debug)]
struct MenuItem {
     name: &'static str,
     price: f32,
}

impl MenuItem {
     fn display(&self) {
         println!("Enjoy {} for only ${:.2}", self.name, self.price);
     }
}

fn main() {
     let steak = MenuItem { name: "Steak", price: 5.0E1 };

     steak.display();
}
```

The pre-defined `'static` lifetime means _alive for the entire program_.




