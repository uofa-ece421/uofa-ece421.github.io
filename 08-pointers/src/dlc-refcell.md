# Doubly-linked Circular List (Take two)

```rust
{{#rustdoc_include ../listings/dlc-strong-weak/src/main.rs:here}}
```
Implement `push` and `pop`

```rust
{{#rustdoc_include ../listings/dlc-strong-weak/src/main.rs:pushpop}}
```

The `fmt::Display` trait was much harder than I thought

```rust
{{#rustdoc_include ../listings/dlc-broken-iterate/src/main.rs:display}}
```

Note the checks that we didn't do excessive cloning

```rust
{{#rustdoc_include ../listings/dlc-drop/src/main.rs:display}}
```

Implement the `Drop` trait to check that ownership is actually working

```rust
{{#rustdoc_include ../listings/dlc-drop/src/main.rs:drop}}
```
