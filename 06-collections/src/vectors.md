# Vectors

A _vector_ is a variable sized array

Vectors are parts of Rust's support for [collections](https://doc.rust-lang.org/std/collections/index.html)

```rust
fn main() {
    let mut v_new = Vec::with_capacity(4);

    v_new.push(0);
    v_new.push(1);
    v_new.push(2);
    v_new.push(3);

    let v_collect: Vec<_> = (0..4).collect();

    assert_eq!(v_new, v_collect);

    let mut v_store = Vec::from([0; 4]);

    for (i, v) in v_store.iter_mut().enumerate() {
        *v = i;
    }

    let v_init = vec![0, 1, 2, 3];

    assert_eq!(v_init, v_store);
}
```

As part of the standard library, vectors have lots of [support](https://doc.rust-lang.org/std/vec/struct.Vec.html)
