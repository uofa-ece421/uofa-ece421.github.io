# Doubly-Linked Circular List (Last Time!)

The [std::Box::into_raw](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw) is important to make sure `Node<T>` is dropped at the right time.

```rust
{{#rustdoc_include ../listings/dlc-raw/src/main.rs:here}}
```
Implement `push` and `pop`

```rust
{{#rustdoc_include ../listings/dlc-raw/src/main.rs:pushpop}}
```

Implement `Drop` for `Token<T>` again to check that we've done it right

```rust
{{#rustdoc_include ../listings/dlc-raw/src/main.rs}}
```
