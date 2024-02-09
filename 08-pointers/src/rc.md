# Rc

* `Rc` is the _reference counting_ smart pointer.
* Like `Box`, it implements both the `Deref` and `Drop` traits, so you can use it (almost) interchangeably with references.
* `Rc` allows multiple *immutable* owners
* `Rc::clone()` is used to increment the owner count - it is a shallow copy

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("Shared string"));

    println!("initial ref count {}", Rc::strong_count(&a));

    {
        let b = Rc::clone(&a);

        println!("a's ref count {}", Rc::strong_count(&a));
        println!("b's ref count {}", Rc::strong_count(&b));

        // Like box, Rc has Deref and Drop traits implemented
        assert_eq!(a.len(), b.len());
        println!("Just checking: {}", b);
    }

    println!("ref count after b is dropped {}", Rc::strong_count(&a));
} // ref count goes to zero when a goes out of scope
```

`Rc` is for single-threaded programs only. For multi-threaded programs, use the
atomic reference counter, [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)

### Weak

[std::rc::Weak](https://doc.rust-lang.org/std/rc/struct.Weak.html) is a version
of `Rc` that holds a _non-owning_ reference to a managed object.

`Weak` pointers can be used as temporary references to `Rc` objects, and in
particular can be used to prevent circular references between `Rc` pointers.
Mutual owning `Rc` references could prevent either object from being dropped.

You can't directly dereference a weak pointer, because the object it referenced
may have been dropped. You can `upgrade()` the weak referece to a _strong_ `Rc`
reference to use it.

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new("shared string".to_string());
    let c = {
        let b = Rc::clone(&a);

        let d = Rc::downgrade(&b); 
        println!("b's counts strong {}, weak {}", Rc::strong_count(&b), Rc::weak_count(&b));
        d
    };

    println!("a's counts strong {}, weak {}", Rc::strong_count(&a), Rc::weak_count(&a));

    if let Some(d) = c.upgrade() {
       println!("can use the upgraded weak pointer: {}", d);
    }

    drop(a);

    match c.upgrade() {
        Some(d) => println!("still here! {}", d),
        None => println!("all is lost!"),
    }
}
```
