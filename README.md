<img src="https://www.seven.io/wp-content/uploads/Logo.svg" width="250" />

# Official API Client for Rust

## Installation

`cargo add seven-client`

### Example

```rust
let client = Client::new("MySecretApiKeyFromSeven.io!".to_string());
println!("Balance: {}", client.get().is_ok());
```

#### Support

Need help? Feel free to [contact us](https://www.seven.io/en/company/contact).

##### License

[![MIT](https://img.shields.io/badge/License-MIT-teal.svg)](LICENSE).
