# Unsafe Operations

### Dereferencing a Raw Pointer

Raw pointers are similar to `Box<T>` except they do not _own_ the data, and
do not automatically implement the `Deref` and `Drop` traits.

Raw pointers are similar to references, except that they are allowed to ignore
the borrowing rule about having a single mutable pointer to some location

```rust
fn main() {
    let mut num: f32 = 3.1415;
 
    let raw_const = &num as *const f32;
    let raw_mut = &mut num as *mut f32;

    unsafe {
        *raw_mut = 1.414;
        println!("raw_const = {}", *raw_const);
    }
}
```

### Calling an Unsafe Function

Functions or methods may be marked as `unsafe` because they are OS calls,
or proprietary libraries possibly written in C. When you call such a function
within an `unsafe` block you are accepting responsibility for understanding
and upholding the function's contract.

```rust
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn puts(s: *const c_char);
}

fn main() {
    let c_str = CString::new("RustaCean!").expect("?!");
    unsafe {
        puts(c_str.as_ptr());
    }
}
```

### Mutable Static Variables

```rustc
extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};

static mut SYSTEM_TIME: SystemTime = UNIX_EPOCH;

fn set_time() {
    unsafe {
        SYSTEM_TIME = SystemTime::now();
    }
}

fn main() {
    set_time();
    let datetime: DateTime<Local>;
    unsafe {
        datetime = SYSTEM_TIME.into();
    }
    println!("{}", datetime.format("%d/%m/%Y %T"));
}
```
