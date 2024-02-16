# Bi-directional Channels

A `channel` is a multiple-producer-single-consumer pipe _in one direction_.
Many concurrent programs are client-server or request-response architectures,
yet Rust core does not support bi-directional channels.

The trick is for the client to include the reply channel in the server request 

```rust
{{#rustdoc_include ../listings/message/src/main.rs}}
```
