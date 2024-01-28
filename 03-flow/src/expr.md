# Statements vs Expressions

**Statements** are instructions that perform some action and do not return a value
  - statements are terminated with a `;`
  - example: `let x = 1;`

**Expressions** are instructions whose action results in a value
  - expressions do not end with a `;`
  - example: `if x > 0 { x = 0; }`

```rust
fn main() {
    let x = {  // a block expression
        let x = 3;
        x + 1  // results in this value
    };

    // macro evaluation is an expression, but the result is ignored by making it a statement
    println!("The value of x is: {x}");
}
```

Rust is an expression-based language, so there are more expression types than statement types 
