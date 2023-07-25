# Zero Rust SDK
Rust SDK for [Zero](https://tryzero.com). Provides a clear and simple interface for accessing the secrets manager GraphQL API.

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
zero-sdk = "0.2.0"
```

## Usage
```rust
use zero_sdk::{Arguments, Zero};

let secrets = Zero::new(Arguments {
    pick: Some(vec![String::from("my-secret")]),
    token: String::from("my-zero-token-from-env"),
    caller_name: Some(String::from("cicd")),
})
.unwrap()
.fetch();
```
