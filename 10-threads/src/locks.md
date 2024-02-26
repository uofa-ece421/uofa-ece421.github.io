# Using Locks

A `Mutex<T>` is a wraps a smart pointer called a `MutexGuard`, which implements
the `Deref` and `Drop` traits. Rust supports for data integrity:
* must call `lock` before accessing the data protected by the mutex
* lock is automatically released when owner exits scope

Rust is conservative about which data needs to be locked (e.g. globals).
Note that locks, threads, etc are provided by the runtime - they are not
intrinsically part of the language itself. Although Rust's ownership rules
go a long way, Rust cannot detect deadlock or fix your performance problems!

```rust
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Mutex::new(0);
    let handle = thread::spawn(move || {
        for i in 1..10 {
            {
                let mut val = counter.lock().unwrap();
                *val = i;
            }
            println!("number {:?} from the spawned thread!", counter);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    //println!("final number {:?} from main", counter);
}
```

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### `Send` and `Sync`

*Send* is a marker trait that indicates that _ownership_ of the value can be
`move`d between threads.

* Almost every Rust type is `Send`, except for types explicity documented as not being thread-safe, e.g. `Rc<T>` and `RefCell<T>`
* Any compound type composed entirely of `Send` types is also `Send`.
* A type _not_ marked `Send` (e.g. raw pointers) can be passed between threads by writing an `unsafe` trait. In this case, the implementor is taking responsibility for making sure the transfer occurrs correctly.

*Sync* is a marker trait that indicates that the value can be referenced from
multiple threads.

* A type `T` is `Sync` if `&T` is `Send`.
* Any compound type composed entirely of `Sync` types is also `Sync`.
* You can also implement `unsafe` traits for types not marked `Sync`.

