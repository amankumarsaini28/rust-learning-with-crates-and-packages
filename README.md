# Learning Rust: Creates & Packages

## Demo Code for a rust app with external crates as packages

- parent cargo defines workspace
- binary package to run app
- lib packages imported by one another and binary package

## Appendix

### Install Rust

```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Build Project

````bash
cargo build
````

### Run Project

```bash
cargo run --package app
```

### Importing external crate

```rust
extern crate <name>;

use name::<mod>;
```