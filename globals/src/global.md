# Global Memory in Rust

For some reason, Rust does not seem to like Globals, which can be somewhat
perplexing to non-Rustaceans

## What is Global Memory?

Typically, Global/Static memory is neither stack nor heap, it is in the
_initialized data_ section of the binary. Technically, read-only globals,
e.g. string constants, can reside in a _text_ section, and uninitialized
globals are in the _BSS_ section and are initialized to zero. Because global
memory is part of the binary, it always exists and cannot be created or
destroyed.

The difference between global and static memory is visibility:
* global memory can be referenced from all scopes in the program, including the global scope
* static memory is visible only within the scope in which it is declared, e.g. a function or file. Of course, you may be able to cheat by sharing the address of static memory outside of its declared scope.

```rust
static mut DBG_LEVEL: u8 = 1;     // Type must be specified
static PRODUCTION: bool = false;  // Initial value must be specified

fn main() {
   if PRODUCTION {
       DBG_LEVEL = 0;
   }
   println!("Running at debug level {}", DBG_LEVEL);
}
```

The rumours are true: Rust does not like _mutable_ global memory!
