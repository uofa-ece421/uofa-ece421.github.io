# Synchronized Global Memory

Of course, Rust complains about `static mut` globals because it can't prove
that access to the data is safe, since the data is (potentially) visible in
many scopes that could be executing concurrently.

```rust
use std::sync::atomic::{AtomicU8, Ordering};

static DBG_LEVEL: AtomicU8 = AtomicU8::new(1);
static PRODUCTION: bool = true;  // Initial value must be available at compile time

fn main() {
   if PRODUCTION {
       DBG_LEVEL.store(0, Ordering::Relaxed);
   }
   println!("Running at debug level {}", DBG_LEVEL.load(Ordering::Relaxed));
}
```

Note this is an exampe of interior mutability.
For complex types, the thread-safe way to get interior mutability is
[std::sync::Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) or
[std::sync::RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html).

```rustc
extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::RwLock;

static SYSTEM_TIME: RwLock<SystemTime> = RwLock::new(UNIX_EPOCH);

fn set_time() {
   let mut time = SYSTEM_TIME.write().unwrap();
   *time = SystemTime::now();
   // the lock is released when time goes out of scope
}

fn main() {
    set_time();
    let time: SystemTime = *SYSTEM_TIME.read().unwrap();
    let datetime: DateTime<Local> = time.into();

    println!("{}", datetime.format("%d/%m/%Y %T"));
}
```

For sequential programs you can also tell Rust the data isn't being shared:

```rustc
extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cell::RefCell;

thread_local!(static SYSTEM_TIME: RefCell<SystemTime> = RefCell::new(UNIX_EPOCH));

fn set_time() {
   SYSTEM_TIME.with(|time| {
      *time.borrow_mut() = SystemTime::now();
   });
}

fn main() {
    set_time();
    let time: SystemTime = SYSTEM_TIME.with(|time| *time.borrow());
    let datetime: DateTime<Local> = time.into();
    
    println!("{}", datetime.format("%d/%m/%Y %T"));
}
```
