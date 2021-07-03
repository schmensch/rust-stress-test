# Rust Stress Test

A small program to fully saturate all available CPU resources, written in Rust.

## Compile and run

### Prerequisites

- Rust (<https://www.rust-lang.org/>)
- Cargo (Rust package manager, included by default)

### Compile

#### Quick compile

You need to clone this repository, then open a terminal in the cloned repository and type `cargo build`. The binary will be in `target/debug/stresstest`, or in `target/debug/stresstest.exe` if you're on Windows.

### Better compile

You can compile code more efficently, resulting in smaller binary sizes and faster execution. To do so, use `cargo build --release`.

### Even more optimisation

I optimize the release binaries with GNU Strip. This is really good for file size, cutting the binary size from 3.2MB to 270KB. To do so, do `strip <binary>`, replacing binary with the binary name.
