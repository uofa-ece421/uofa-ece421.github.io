# Lifetime Elision

Every reference in Rust has a lifetime, and Rust always checks the owner's
lifetime to ensure there are no dangling pointers.

Before Rust version 1.0 every reference required an explicit lifetime.
However, the compiler can now infer lifetimes in several situations so that you
don't always have to specify the lifetime for every reference.

### Lifetime Elision Rules


1. The compiler first assigns a lifetime identifier to each parameter that is a reference:
    * `fn max<'a, 'b>(x: &'a i32, y: &'b i32)`
1. If there is only one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
    * `fn one<'a>(i: &'a i32) -> &'a i32`
1. If there are multiple input parameters but one of them is `&self` or `&mut self`, then the lifetime of `self` is applied to all output lifetime parameters.

```rust
fn show_max(x: &f32, y: &f32) {
    let op = if x >= y {
        ">="
    } else {
        "<"
    };
    println!("{0} {1} {2}", x, op, y);
}

fn first_word(sentence: &str) -> &str {
    for (i, &c) in sentence.as_bytes().iter().enumerate() {
        if c == b' ' {
           return &sentence[..i];
        }
    }
    &sentence
}

fn ref_max<'a>(x: &'a f32, y: &'a f32) -> &'a f32 {
    if x >= y {
        x
    }
    else {
        y
    }
}

fn main() {
    let x = 1.414;
    let y = 3.141;
    let words = String::from("compare x and y");

    show_max(&y, &x);
    println!("first word: {}", first_word(&words));
    println!("max is {}", ref_max(&y, &x));
}
```

        


        
