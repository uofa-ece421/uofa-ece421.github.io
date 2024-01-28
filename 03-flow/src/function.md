# Functions

* Functions are declared using the `fn` keyword
* Functions may or may not return a value
  - the type of a return value is declared using `->`
* Functions may take parameters
  - the type of every parameter _must_ be declared
* Functions do not have to be declared before they are used
  - function declarations must be visible within the scope of the caller
* Rust code uses _snake case_ for function and variable names

A function _declaration_ is a statement; a function _evaluation_ is an expression

```rust
// Functions with no return value actually return the unit type `()`
fn main() -> () {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

The requirement to declare all parameter types is a Rust design decision.
It allows function calls to form part of the type inference infrastructure.
  