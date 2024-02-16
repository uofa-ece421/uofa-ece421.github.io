# Channels

_Channels_ are used for _message passing_ between threads.
From the Go language documentation:

> Do not communicate by sharing memory; instead, share memory by communicating.

Back in the day, there was much ado about the _duality_ of message passing vs
shared memory. The Linda language tried to implement a distributed shared memory
abstraction using message passing.

Today, most message passing APIs are implemented using shared memory. The main
reason: *copying memory is expensive*. It is _very_ difficult for the advantages
of thread code and data locality to overcome the costs of the copy.

Message passing is usually used in concurrent (vs parallel) systems, and of
course heavily used in distributed systems.

A Rust [`mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) is almost exactly like a Unix domain socket or pipe.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();  // so we can have two senders
    thread::spawn(move || {
        let start = Instant::now();
        for _ in 1..5 {
            let msg = format!(" <t1: elapsed {:?}> ", start.elapsed());
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from(" #t2# "),
            String::from(" #t2 again# "),
            String::from(" #t2 more# "),
            String::from(" #t2 last# "),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{}", received);
    }
}
```
