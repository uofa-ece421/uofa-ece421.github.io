# Ownership and Functions

"Call-by-value" parameters are exactly the same as stack values
- primitve types with the _Copy_ trait are copied
- dynamic types are _Moved_

```rust
fn main() {
    let s = String::from("eggs");

    let scrambled = cooks_eggs(s);

    println!("consumed and produced: {}", scrambled);

    let x = 12;

    counts_eggs(x);

    println!("Still here! {x}");
}

fn cooks_eggs(eggs: String) -> String {
    println!("cooking: {}", eggs);

    String::from("GegS")
}

fn counts_eggs(amt: u16) {
    println!("counting {}", amt);
}
```
