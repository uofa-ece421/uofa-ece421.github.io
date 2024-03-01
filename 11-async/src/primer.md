# `async`/`.await`

* Preserves the look and feel of synchronous programming (to some extent)
* Zero-cost abstraction - pay for what you use
* Futures are inert, in that they make progress only when polled
* No built-in runtime. A runtime is needed to do the multitasking, but you can use any single- or multi-threaded crate that you want
* the API is still evolving, although it is becoming more stable over time 

Whereas a synchronous function will block the entire thread when a blocking
call is made, an `async` function will yield to other tasks at `await` points.

```rust
extern crate futures;

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
```

Note that futures are _lazy_ - they won't run/poll until you tell them to

```rust
{{#rustdoc_include ../listings/sleep-sync/src/main.rs}}
```

Here is the same program except that `sleeping` and `interrupting` run at the
same time instead of one at a time.

```rust
{{#rustdoc_include ../listings/sleep-async/src/main.rs}}
```

Note that the differences between the synchronous and asynchronous programs are
minimal, essentially annotations of what can run concurrently and where 
blocking occurs.

