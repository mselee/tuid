# tuid-rs
Sequential UUID generator.

This generator uses a time-based component as the shared prefix.
The prefix is non-monotonic, and wraps around every once in a while to achieve a dense key space.

## Installation
If using [cargo-edit](https://github.com/killercup/cargo-edit)
```shell
cargo add tuid
```
or add it yourself to `Cargo.toml`
```toml
[dependencies]
tuid = "0.1.0-alpha"
```

## Usage
```rust
fn main() {
  use rand::Rng;

  let mut rng = rand::thread_rng();
  let tid = tuid::gen::default(rng.gen(), rng.gen());
  let uid = tid.as_uuid();
  let hex = tid.as_hex();
}
```
