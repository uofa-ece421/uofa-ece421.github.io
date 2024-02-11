# The Dark Side

References:
* [The Rustonomicon](https://doc.rust-lang.org/nightly/nomicon/)
* [Unofficial Rust](https://github.com/rust-unofficial)
* [Rust Book - Chapter 19.1](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

** Unsafe Rust

Unsafe Rust exists because compiler analysis _must_ be conservative.
The compiler must reject possibly valid programs when it can't prove the
code is safe.

Using `unsafe` is your promise to the compiler that you understand and take
responsibliity for what you are doing.

### Unsafe Superpowers

To switch to unsafe Rust, use the `unsafe` keyword and then start a new block
that holds the unsafe code. You can take five actions in unsafe Rust that you
can’t in safe Rust, which we call *unsafe superpowers*. Those superpowers
include the ability to:

* Dereference a raw pointer
* Call an unsafe function or method
* Access or modify a mutable static variable
* Implement an unsafe trait
* Access fields of `union`s

`unsafe` blocks *do not* turn off the borrow checker or disable any other of
Rust's safety checks.

Typically, unsafe code is wrapped in a safe abstraction with a safe API so
that it is isolated as much as possible, which simplifies debugging.
