//! Example: Basic JSON-RPC client usage
//!
//! This example demonstrates how to create a throttled JSON-RPC client
//! for a Bitcoin-like daemon.

use throttled_json_rpc::jsonrpc_client;

// Define the RPC client with the methods you need
jsonrpc_client!(pub struct BitcoinClient {
    single:
        /// Get the current block count
        pub fn getblockcount(&self) -> Result<u64>;

        /// Get block hash by height
        pub fn getblockhash(&self, height: u64) -> Result<String>;

        /// Get blockchain info
        pub fn getblockchaininfo(&self) -> Result<serde_json::Value>;
    enum:
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with throttling parameters:
    // - 5 max concurrent requests
    // - 10 requests per second limit
    // - No batching (0 = disabled)
    let client = BitcoinClient::new(
        "http://localhost:8332".to_string(),
        Some("rpcuser".to_string()),
        Some("rpcpass".to_string()),
        5,  // max_concurrency
        10, // rps (requests per second)
        0,  // max_batch_size (0 = no batching)
    );

    println!("Fetching blockchain info...");

    // Make some RPC calls
    match client.getblockcount() {
        Ok(count) => println!("Current block height: {}", count),
        Err(e) => eprintln!("Error getting block count: {}", e),
    }

    // The throttling ensures we don't overwhelm the server
    for height in 0..5 {
        match client.getblockhash(height) {
            Ok(hash) => println!("Block {} hash: {}", height, hash),
            Err(e) => eprintln!("Error getting block {}: {}", height, e),
        }
    }

    println!("\nNote: This example requires a running Bitcoin node");
    println!("with RPC server enabled on localhost:8332");

    Ok(())
}
