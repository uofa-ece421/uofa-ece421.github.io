# Data types

| _Type_ | `size_of::<Type>()` | example literal |
|--------|----------------------|-----------------|
| `bool` | 1 | `true`,`false` |
| `u8`/`i8` | 1 | `b'A'` |
| `u16`/`i16` | 2 | `0o77` |
| `u32`/`32` | 4 | `0xffffff` |
| `u64`/`i64` | 8 | `100_000_000` |
| `u128`/`i128` | 16 | `1u128` |
| `usize`/`isize` | arch | `1 as usize` |
| `char` | 4 | `'a'`, `'😻'` |
| `f32` | 4 | `1.2e-9` |
| `f64` | 8 | `1.2` |

- `bool` is one byte in Rust, vs _C_, where it is equivalent to int
- integer literal styles are interchangeable, except for `b` (byte)
- `usize`/`isize` width depends on the machine
  * used for indexing (similar to `size_t` in _C_)
- `char` is a Unicode Scalar Value
  * Unicode values range from `U+0000` to `U+10FFFF`
  * Definitely not like an integer (as in _C_)

[Operators](https://doc.rust-lang.org/book/appendix-02-operators.html) are relatively standard






