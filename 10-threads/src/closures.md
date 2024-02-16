# Closures

_Closures_ are anonymous functions, sometimes called _lambda_ functions, that
can be saved in a variable or passed as arguments to other functions.

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }

fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    assert_eq!(add_one_v1(5), add_one_v2(5));
    assert_eq!(add_one_v3(5), add_one_v4(5));

    let ptr: fn(u32) -> u32 = add_one_v1;
    let closure: fn(u32) -> u32 = |x| x + 1;

    assert_eq!(ptr(10), closure(10));
}
```

Functions that expect closures as arguments implement the `FnOnce` Trait
* `FnOnce` as in [Option::unwrap_or_else](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else)
* `FnMut` is a subtrait of `FnOnce`, as in [Iterator::map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
* `Fn` is a subtrait of both `FnOnce` and `FnMut`, and is different that a function pointer `fn` (which is a primitive type).

Closures are typically short, simple and relevant to a specific context, compared
to functions that are usually more general purpose or context insensitive.

### Borrows and Moves

Closures can capture values from their environment in three ways:
immutable borrow, mutable borrow, and move. This is similar to the way function
parameters work, although it now applies to the closure body. The compiler can
usually infer which type of capture to use.

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();

    let mut borrows_mutably = || list.push(4);

    // println!("both a mutable and immutable borrow: {:?}", list);
    borrows_mutably();

    println!("After mutable borrow is done {:?}", list);
}
```

Moving ownership into a closure is often used to pass data to threads,
because a borrow would mean that the data could be accessed concurrently.



