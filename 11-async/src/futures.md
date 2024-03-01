# Futures

The `async` keyword tells the compiler to transform a function into a state
machine that implements the `Future` trait.

```rust

/* A simplified version of the Future trait. The Output type is the function
 * return type. For procedures like main() or back_to_the_future we use ().
 * poll() is used to determine if the Future is ready to complete. The callback
 * function wake() is used by the future to signal the runtime that it is ready.
 */
trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),  // When Ready, return the type T of the function
    Pending,   // The Future is still waiting
}

struct AlwaysReady;

impl Future for AlwaysReady {
    type Output = ();

    fn poll(&mut self, _wake: fn()) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

fn main() {
   back_to_the_future();
}

// async fn back_to_the_future() {...}
fn back_to_the_future() -> impl Future<Output=()> {
    println!("Great Scott! 88 GigaWatts!");
    AlwaysReady
}
```

A more interesting `poll` implementation:

```rust
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl Future for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // The socket has data -- read it into a buffer and return it.
            Poll::Ready(self.socket.read_buf())
        } else {
            // The socket does not yet have data.
            //
            // Arrange for `wake` to be called once data is available.
            // When data becomes available, `wake` will be called, and the
            // user of this `Future` will know to call `poll` again and
            // receive data.
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
```



