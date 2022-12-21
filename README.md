# Zero Rust SDK
Rust SDK for [Zero](https://tryzero.com). Provides a clear and simple interface for accessing the secrets manager GraphQL API.

## Installation
```toml
[dependencies]
rust-sdk = "0.13"
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
