# Doubly-linked Circular List

Why is this my favorite list for systems programming?
* No sentinel (`NULL`, `None`) pointers to worry about
  - Empty list is not a special case
* `prepend` and `append` are exactly the same cost
  - No annoying tail pointer that has to be updated
* Can delete from the middle!
  - Don't need to start at the root and _find_ the node

```rust
{{#rustdoc_include ../listings/dlc-01/src/main.rs:here}}
```

Issue creating DLNodes: need [Weak](https://doc.rust-lang.org/std/rc/struct.Rc.html)


