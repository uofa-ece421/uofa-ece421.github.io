# Hello World

First create a new project using Cargo:

```console
$ cargo new hello_world
$ cd hello_world
```

Inside the directory *hello_world*:

```console
$ ls
Cargo.toml src
```

And inside *src/main.rs*:

```rust
fn main() {
    println!("Hello, world!");
}
```

You can try it out with:

```console
$ cargo build
   Compiling hello_world v0.1.0 (/Users/runrau/src/rust/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 2.84s
```

### A Closer Look

`main`: a function that takes no arguments and has no return value

- However, it can (and does) have side-effects
- The body of the function is inside the curly braces `{` and `}'

`println!`: a macro that formats its arguments and writes the string to `stdout`

- Macros look like functions but have the `!` to identify them
- Most executable statements in Rust are terminated with a semicolon (`;`)
