# Enums that include Data

These are literally a discriminated union

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },  // fields can be named
    Write(String),
    TextColor(i32, i32, i32),
}

impl Message {
    fn send(&self) {
        if let Message::Write(s) = self {
           println!("Sending: {}", s);
        }
    }

    fn change(&mut self, r: i32, g: i32, b: i32) {
        *self = Message::TextColor(r, g, b);
    }
}

fn main() {
    let mut color = Message::TextColor(0x0, 0xff, 0x0);
    let msg = Message::Write(String::from("hello"));

    msg.send();

    color.change(0xff, 0x00, 0x00);
    println!("color {:#?}", color);
}
```