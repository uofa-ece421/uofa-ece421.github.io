# Streams

The `Stream` trait is a `std::iter::Iterator` for Futures.
The definition of [futures::stream::Stream](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) is:

```rust
pub trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;

    // Provided method
    fn size_hint(&self) -> (usize, Option<usize>) { ... }
}
```

Rust does not directly support `async for` loops.

```rust
extern crate tokio;

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}
```

