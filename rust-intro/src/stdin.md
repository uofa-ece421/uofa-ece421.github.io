# Reading from `stdin`

We are going to implement a guessing game.
First create a new project with the following *src/main.rs*:

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

Lots of new things to note:
- `use std::io` - Rust equivalent of `#include <stdio.h>`
- variables are immutable by default. Use `mut` to change their value
- Rust is strongly typed, but makes extensive use of type inferencing
- Rust has some object oriented features, but not full classes as in C++
- `stdin`, `stdout`, and `stderr` are present and ready to use, as in C

A closer look at [`read_line`](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)

