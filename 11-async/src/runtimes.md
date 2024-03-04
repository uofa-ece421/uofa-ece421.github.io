# Runtimes

Rust does not have a builtin _runtime_, but all `async` programs require one.
Both single- and multi-threaded runtimes are available:
* [tokio](https://crates.io/crates/tokio) - very popular, especially for sockets
* [async-std](https://crates.io/crates/async-std) - wraps the `std` library calls
* [fuchsia-async](https://fuchsia.googlesource.com/fuchsia/+/master/src/lib/fuchsia-async/) - an executor for Google's Fuchsia OS.

The runtimes will usually require that you enable some futures.

```cargo
[dependencies.async-std]
version = "1.12.0"
features = ["attributes"]
```

The runtimes can be explicitly started:

```rust
extern crate tokio;

async fn app() {
    println!("tokio ran me");
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = app();
    rt.block_on(future);
}
```

or the runtimes can be started using decorators:

```rust
#[tokio::main]
async fn main() {

}
```

