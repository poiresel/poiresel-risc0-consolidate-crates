# Updated RISC Zero Rust Starter Template - Using Factor Examples


[WIP] please note code is still WIP and is incomplete 
This code serves as an example of attemping to remove dependency on the methods crate. There is a single workspace and crate for both the host and guest code. The build.rs file does still exist. The code attemptes to utilize building the binaries first for the guest code and then using those binaries for the host code

## Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following command:

```
cargo run
```

