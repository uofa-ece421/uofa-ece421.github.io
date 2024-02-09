# Box

`Box`, `Rc`, and `Arc` are Rust's _smart pointers_

```rust
fn main() {
    let x: i32 = 101;
    let boxed = Box::new(x);
    let borrowed = &x;

    println!("Display is handled {} = {}", borrowed, boxed);

    assert_eq!(x, *boxed);
    assert_eq!(x, *borrowed);
    // boxed is dropped here
}
```

`Box` is considered _smart_ because it implements the `Deref` and `Drop` traits.
This allows boxed pointers to be behave like references.

_Deref Coercion_ uses the `Deref` trait to convert references from one type
to another, e.g. `&String` to `&str`, or `Vec<T>` and `&[T]`.

Deref coercion lets us write code that makes a lot of `&` and `*` operators
implicit, and makes the code more portable between references and smart pointers.


```rust
fn main() {
    let x = 101;
    let borrowed = &x;
    let boxed = Box::new(x);

    println!("borrowed is a pointer {:?} = {:p}", borrowed as *const i32, &x);
    println!("Box points to a copy: {:?}", Box::into_raw(boxed));
    // This is also a leak because boxed has now been consumed
}
```

