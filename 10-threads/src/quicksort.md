# Parallelizing Quicksort

### Sequential Quicksort

Works by partitioning the data into two halves, such that the values below the
_pivot_ index are all less than the values above the pivot.

Then it recursively calls itself on the two partitions.

The average complexity is <var>O(n*log<sub>2</sub>n)</var> and the worst case
is <var>O(n<sup>2</sup>)</var>, which happens when the data is already sorted.

```rust
{{#rustdoc_include ../listings/quicksort/src/main.rs:sequential}}
```

Since the two halves are disjoint we don't even need locks!

```rust
{{#rustdoc_include ../listings/quicksort/src/main.rs:parallel}}
```

To get around the lifetime problem, use _scoped_ threads from `crossbeam`

```rust
{{#rustdoc_include ../listings/quicksort-scoped/src/main.rs:parallel}}
```

