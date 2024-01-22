# Error Handling

### Results must be used

Another important safety feature of Rust is the `#[must_use]` attribute, which
will cause the compiler to issue a warning when an annotated object is ignored.

```rust
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut handle = File::open("hello.txt").unwrap();

    let mut data = vec![];
    handle.read_to_end(&mut data);

    println!("read: {:?}", data);

    Ok(())
}
```

Lots of different ways to handle errors:
- `unwrap()` - causes a panic if Err returned
- `expect("message")` - prints "message" if Err returned
- `assert!(handle.read_to_end(&mut data).is_ok())` - panics if Err returned
- use '?', which propagates the error up the call stack
- use `match` or `if let` to handle both success and failure cases

```rust
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut handle = match File::open("hello.txt") {
        Ok(fd) => fd,
        Err(e) => {
            println!("Open failed: {}", e);
            return Err(e)
        },
    };

    let mut data = vec![];
    handle.read_to_end(&mut data)?;

    println!("read: {:?}", data);

    Ok(())
}
```

