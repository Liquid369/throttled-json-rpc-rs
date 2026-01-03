# throttled-json-rpc

[![Crates.io](https://img.shields.io/crates/v/throttled-json-rpc.svg)](https://crates.io/crates/throttled-json-rpc)
[![Documentation](https://docs.rs/throttled-json-rpc/badge.svg)](https://docs.rs/throttled-json-rpc)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A macro-based JSON-RPC client generator with built-in rate limiting, concurrency control, and request batching.

## Features

- **Declarative API**: Define your RPC client with a simple macro syntax
- **Rate Limiting**: Control requests-per-second (RPS) to avoid overwhelming servers
- **Concurrency Control**: Limit simultaneous in-flight requests
- **Request Batching**: Efficiently batch multiple RPC calls
- **Flexible Response Types**: Support for both single-type and enum variant responses

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
throttled-json-rpc = "0.1"
```

## Quick Start

```rust
use throttled_json_rpc::jsonrpc_client;

jsonrpc_client!(pub struct MyRpcClient {
    single:
        /// Get block hash by height
        pub fn getblockhash(&self, height: u64) -> Result<String>;
        
        /// Get block by hash
        pub fn getblock(&self, hash: String, verbosity: u32) -> Result<serde_json::Value>;
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MyRpcClient::new(
        "http://localhost:8332".to_string(),
        Some("rpcuser".to_string()),
        Some("rpcpass".to_string()),
        5,    // max 5 concurrent requests
        10,   // max 10 requests per second
        0,    // no batching (0 = disabled)
    );

    let block_hash = client.getblockhash(100)?;
    let block = client.getblock(block_hash, 1)?;
    
    println!("Block: {:?}", block);
    Ok(())
}
```

## Documentation

For detailed documentation and more examples, visit [docs.rs/throttled_json_rpc](https://docs.rs/throttled_json_rpc).

## Throttling Behavior

- **RPS (Requests Per Second)**: When set > 0, ensures minimum time between requests
- **Max Concurrency**: When set > 0, limits how many requests can be in-flight simultaneously
- **Batching**: When set > 0, automatically batches requests up to the specified size

## License

Licensed under the MIT license. See [LICENSE](LICENSE) for details.

## Maintenance

This is a maintained fork of the original `throttled-json-rpc-rs` crate, modernized for current Rust standards and actively maintained by PIVX Labs.

## Contributing

Contributions welcome! Please open issues or PRs on [GitHub](https://github.com/DR-BoneZ/throttled-json-rpc-rs).
