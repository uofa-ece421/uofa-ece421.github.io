# Spawning Tasks

Futures are inert and only advance when polled. To achieve <var>N</var>
concurrent operations, you need to spawn <var>N-1</var> tasks.

### Join

In the example below, we should be able to dance while we learn to sing

```rust
{{#rustdoc_include ../listings/sing/src/main.rs:sing}}
```

But unless we use something like `spawn` or `join!` they won't occur at the
same time:

```rust
{{#rustdoc_include ../listings/sing/src/main.rs:async}}
```

### CPU-intensive tasks

Blocking or CPU-intensive tasks should not be run as `async`. Instead, you can
use [task::spawn_blocking](https://dtantsur.github.io/rust-openstack/tokio/task/fn.spawn_blocking.html) to offload work onto an OS thread:

```rust
extern crate tokio;

use tokio::task;

fn fib_cpu_intensive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib_cpu_intensive(n - 1) + fib_cpu_intensive(n - 2),
    }
}

#[tokio::main]
async fn main() {
    let threadpool_future = task::spawn_blocking(||fib_cpu_intensive(30));
    println!("now we can do other things");
    let n = threadpool_future.await.unwrap();
    println!("answer is {n}");
}
```

(from https://blog.logrocket.com/a-practical-guide-to-async-in-rust/)

### Select

`select` waits on multiple futures and returns as soon as *one* of the tasks
completes. The other tasks are *dropped*. In that sense, `join` is like
<var>AND</var> and `select` is like <var>OR</var>.

```rust
extern crate tokio;

use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}
```

(from https://tokio.rs/tokio/tutorial/select)

### Spawn

And last but not least, the canonical echo server :-)

```rust
{{#rustdoc_include ../listings/echo/src/main.rs}}
```





