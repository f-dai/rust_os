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
* https://os.phil-opp.com - The "Blog OS" created with Rust