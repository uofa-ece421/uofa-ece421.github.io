# std::future::Future

The full `Future` is defined here [std::future::Future](https://doc.rust-lang.org/std/future/trait.Future.html)

```rust
trait Future {
    type Output;
    fn poll(
        // Note the change from `&mut self` to `Pin<&mut Self>`:
        self: Pin<&mut Self>,
        // and the change from `wake: fn()` to `cx: &mut Context<'_>`:
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
```

* [std::pin::Pin](https://doc.rust-lang.org/std/pin/struct.Pin.html) - a wrapper around a pointer that _pins_ its value in place, preventing the value from being moved unless it implements `Unpin`.
* [std::task::Context](https://doc.rust-lang.org/std/task/struct.Context.html) used by the run-time to save and restore state, and to provide access to a `&Waker`

```rust
{{#rustdoc_include ../listings/sleep-future/src/main.rs:sleep}}
```

We can test the `poll` implementation using the `async_std` runtime:

```rust
{{#rustdoc_include ../listings/sleep-future/src/main.rs:main}}
```
