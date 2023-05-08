# Rust OS

Use Rust to build an operation system from scratch. Since I am pretty new to Rust, expect exccessive comments in codes.

## Compilation
```bash
rustup override add nightly
rustup component add rust-src
cargo build
```
The default target json file is specified in `.cargo/config.toml`.

## Documentation
My detailed notes and documentations are located in doc/ directory.

## References
* https://doc.redox-os.org/book/ch00-00-introduction.html - The Redox Operating System
* https://os.phil-opp.com - The "Blog OS" created with Rust
* https://www.rust-lang.org/learn - Official website of Rust programming language
* https://doc.rust-lang.org/rust-by-example/ - Rust by example