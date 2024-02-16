# Concurrency vs Parallelism

Rust supports multiple threads of execution in a single address space.
* *Native* threads in Rust wrap the OS processes 1:1. These threads are capable of true parallelism/concurrency because they can be running on multiple physical CPUs simultaneously.
* *Light-weight* threads are also supported in Rust. These run-times multiplex multiple logical threads on a single native thread.

The words concurrent and parallel are often used interchangeably, and from Rust's point of view have similar safety and memory sharing implications.
* *Concurrent* programs typically use mutiple threads for scaling or functional decomposition (e.g. microservices). The threads are largely independent even if they are performing the same function
  - For example, _Netflix_ might use thousands of threads to stream media to its customers.
  - Data is often _moved_ between threads, e.g. SQL queries
* *Parallel* programs use multiple threads to solve a problem faster. The threads usually have to communicate and coordinate to decompose the data space and maintain its integrity
  - For example, IBM's _Deep Blue_ employed thousands of threads to search the game tree in its chess player.
  - Data is often _shared_ between threads, e.g. matrices in numerical appliations


