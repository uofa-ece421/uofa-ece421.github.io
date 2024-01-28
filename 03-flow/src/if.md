# `if` Expressions

```rust
fn gcd(n: i32) -> i32 {
    let d = if n % 4 == 0 {  // Must be a boolean expression
                4
            } else if n % 3 == 0 {
                3
            } else if n % 3 == 0 {
                2  // all branches must return the same type
            } else {
                1
            };  // semi-colon required when used as an expression
    d
}

fn main() {
    println!("The GCD of 6 is {}", gcd(6));
}
```

Because `if` is an expression we no longer need:

```rust
fn positive(val: i32) -> bool {
    let result;  // declaration without initialization 😱
    if val >= 0 {
        result = true;
    } else {
        result = false;
    }  // no semi-colon when used as a statement
    result
}

fn main() {
    println!("I'm positive about this: {}", positive(-3));
}
