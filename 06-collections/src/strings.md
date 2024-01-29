# Strings

Rust _String_ objects are all part of the common collections

A `String` is a `Vec<u8>` that is guarantreed to be a valid UTF-8 sequence,
but is not null terminated.

```rust
use std::iter::FromIterator;

fn main() {
   let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
   let mut chars: Vec<char> = pangram.chars().collect();

   chars.sort();
   chars.dedup();
   let char_string = String::from_iter(chars);

   let mut alphabet = String::from_utf8(vec![b' ', b'a', b'b', b'c', b'd']).unwrap();
   assert_eq!(char_string[0..5], alphabet);

   alphabet.push_str(&char_string[5..]);
   println!("ALPHABET: {}", alphabet.to_uppercase());
}
```

Generally, use backslash (`\`) to escape characters

```rust
fn main() {
    // You can use escapes to write bytes by their hexadecimal values...
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );


    let long_string = "The linebreak and indentation here ->\
                       <- can be escaped too!";
    println!("{}", long_string);
}
```

For dealing with OS calls, you may need a byte_string

```rust
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let bytes: &[u8; 12] = b"byte string\n";
    io::stdout().write_all(bytes)?;

    let string = String::from("type String");
    io::stdout().write_all(string.as_bytes())?;

    Ok(())
}
```

See [std::string::String](https://doc.rust-lang.org/std/string/struct.String.html) for all the methods available
