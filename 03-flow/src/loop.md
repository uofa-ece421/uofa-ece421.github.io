# `loop` Expressions

```rust
fn main() {
    let mut counter = 0;

    let result = loop {  // the basic loop is infinite
        counter += 1;

        if counter == 10 {
            break counter * 2;  // the (optional) expression is the return value
        }
    };

    assert_eq!(result, 20);
}
```

### Loop Labels

```rust
#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
```