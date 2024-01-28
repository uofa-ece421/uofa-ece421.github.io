# Using `rand`

Now we want to generate a secret number: <https://crates.io>

Add this to `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

And use it in *src/main.rs*:

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```
