# Dynamic Initialization of Globals

Once last river to cross...

```rust
use std::sync::Mutex;

static SYSTEM_STATUS: Mutex<String> = Mutex::new(String::from("Initializing"));

fn set_status(msg: &str) {
    *SYSTEM_STATUS.lock().unwrap() = msg.to_string();
}

fn main() {
    set_status("Ready");

    println!("System Status: {}", *SYSTEM_STATUS.lock().unwrap());
}
```

Rust chose not to follow the C++ approach of static object initialization.
Instead, Rust allows the _construct on first use_ idiom.

```rust
extern crate once_cell;

use std::sync::Mutex;
use once_cell::sync::Lazy;

static SYSTEM_STATUS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("Initializing")));

fn set_status(msg: &str) {
    *SYSTEM_STATUS.lock().unwrap() = msg.to_string();
}

fn main() {
    set_status("Ready");

    println!("System Status: {}", *SYSTEM_STATUS.lock().unwrap());
}
```

or

```rust
extern crate lazy_static;

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SYSTEM_STATUS: Mutex<String> = Mutex::new(String::from("Initializing"));
}

fn set_status(msg: &str) {
    *SYSTEM_STATUS.lock().unwrap() = msg.to_string();
}

fn main() {
    set_status("Ready");

    println!("System Status: {}", *SYSTEM_STATUS.lock().unwrap());
}
```
