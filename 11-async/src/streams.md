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

Rust does not directly support `async for` loops, but you can emulate them with
streams:

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

The [futures::stream::StreamExt](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html)
provides a variety of combinators, including `for_each_concurrent`:

```rust
use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    stream::iter(0..200u64)
        .for_each_concurrent(20, |number| async move {
            let mut rng = thread_rng();
            let sleep_ms: u64 = rng.gen_range(0..20);
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
            println!("{}", number);
        })
        .await;
}
```

(example from: https://kerkour.com/rust-async-combinators)



