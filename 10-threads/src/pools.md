# Thread Pools

The idea is to amortize the cost of thread creation by doing it once at the
beginning and then reusing the body for different tasks:

```rust
{{#rustdoc_include ../listings/quicksort-pool/src/tpool.rs:worker}}
```

The actual pool is just a collection of workers:

```rust
{{#rustdoc_include ../listings/quicksort-pool/src/tpool.rs:pool}}
```

The pool is used in the main program:

```rust
{{#rustdoc_include ../listings/quicksort-pool/src/main.rs:pool}}
```




