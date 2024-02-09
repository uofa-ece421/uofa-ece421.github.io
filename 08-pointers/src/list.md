# Self-referential Structures

`Box` is used to make _recursive_ structures a fixed/known size.

```rust
use std::fmt;

struct SLNode<T> {
    next: Option<Box<SLNode<T>>>, // Option needed because the last element's next is None
    val: T,
}

impl<T> SLNode<T> {
    fn new(val: T, next: Option<Box<SLNode<T>>>) -> SLNode<T> {
        SLNode {
            next,
            val,
        }
    }
}   

struct SList<T> {
    root: Option<Box<SLNode<T>>>,
}

impl<T> SList<T> {
    fn new() -> SList<T> {
        SList { root: None }
    }
}

impl<T: std::fmt::Display> fmt::Display for SList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut p = &self.root;
        while let Some(curr) = p {
            write!(f, "{} ", curr.val)?;
            p = &curr.next;
        }
        write!(f, "]")
    }
}  
    
fn main() {
    let mut stack = SList::<i32>::new();

    println!("our first list based stack: {}", stack);
}
```

Let's add the `push` method

```rust
{{#rustdoc_include ../listings/list-02/src/main.rs:here}}
```

The problem is that `new` consumes it's arguments, i.e. it takes ownership away
from `self.root`.

The secret is [std::mem::replace](https://doc.rust-lang.org/std/mem/fn.replace.html)

```rust
{{#rustdoc_include ../listings/list-push-works/src/main.rs:here}}
```

If `push` was hard, `pop` is not going to be any easier

```rust
{{#rustdoc_include ../listings/list-pop-broken/src/main.rs:here}}
```

Now we need both `replace` and [std::mem::swap](https://doc.rust-lang.org/std/mem/fn.swap.html). We also need the `Copy` trait to return the value.

```rust
{{#rustdoc_include ../listings/list-pop-works/src/main.rs:here}}
```





