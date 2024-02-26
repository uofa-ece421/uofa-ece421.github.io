# Benchmarking and Tuning

Let's see what awesome speedup we get!

First, setup a test harness

```rust
{{#rustdoc_include ../listings/quicksort-scoped/src/main.rs:bench}}
```

Add some instrumentation

```rust
{{#rustdoc_include ../listings/quicksort-bench/src/main.rs:instrument}}
```
