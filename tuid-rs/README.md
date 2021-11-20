# tuid-rs

## Installation

If using [cargo-edit](https://github.com/killercup/cargo-edit)
```shell
cargo add tuid
```
or add it yourself to `Cargo.toml`
```toml
[dependencies.tuid]
version = "0.1"
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

---
