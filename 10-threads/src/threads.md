# Introduction to Threads

Use [thread::spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html) to create a thread:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread count {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main count {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().expect("child panicked!");
}
```

You can move data into and out of threads:

```rust
use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let thread_output = thread::spawn(move || {
        println!("I own this: {:?}", list);
        list.push(4);
        list
    });

    //println!("You can't print list here {:?}", list);
    
    if let Ok(list) = thread_output.join() {
        println!("And now main owns it again {:?}", list);
    }
}
```



