# Rust Unsafe Pointer and Ownership Bug

This repository demonstrates a common error in Rust when working with raw pointers and unsafe code. The error arises from modifying a vector's contents through a raw pointer after the vector's ownership has been dropped. This leads to undefined behavior, potentially causing crashes or data corruption.  The solution illustrates safe and correct approaches using proper ownership and borrowing.

## How to Reproduce

1. Clone this repository.
2. Run `cargo run`.
3. Observe the runtime error.