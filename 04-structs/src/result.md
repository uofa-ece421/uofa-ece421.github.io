# Result

`Result<T, E>` is the type used for returning and propagating errors

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

_Result_ is used a lot for I/O and OS calls. Like _Option_, _Result_ is so
pervasive it has a vast support [library](https://doc.rust-lang.org/std/result/)

```rust
fn main() {
    let happy: Result<i32, i32> = Ok(10);
    let sad: Result<i32, i32> = Err(10);

    assert!(happy.is_ok() && !happy.is_err());

    if let Ok(val) = happy {
        println!("The happy value is {val}");
    }

    let rc = match sad {
        Ok(v) => v,
        Err(e) => -e,
    };
    println!("Am I positive about sad? {rc}");
}
```
