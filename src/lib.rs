//! # Throttled JSON-RPC Client
//!
//! A macro-based JSON-RPC client generator with built-in rate limiting,
//! concurrency control, and request batching.
//!
//! ## Features
//!
//! - **Declarative API**: Define your RPC client with a simple macro syntax
//! - **Rate Limiting**: Control requests-per-second (RPS) to avoid overwhelming servers
//! - **Concurrency Control**: Limit simultaneous in-flight requests
//! - **Request Batching**: Efficiently batch multiple RPC calls
//! - **Flexible Response Types**: Support for both single-type and enum variant responses
//!
//! ## Throttling Behavior
//!
//! This library provides **synchronous/blocking** throttling using `std::thread::sleep`:
//!
//! ### Rate Limiting (RPS)
//! - **When**: `rps > 0`
//! - **How**: Enforces minimum time `1/rps` seconds between consecutive requests
//! - **Behavior**: Thread sleeps if previous request was too recent
//! - **Scope**: Global across all threads using the same client instance
//!
//! ### Concurrency Limiting
//! - **When**: `max_concurrency > 0`
//! - **How**: Limits number of simultaneous in-flight requests
//! - **Behavior**: Thread blocks (via Condvar) until a slot is available
//! - **Scope**: Global across all threads using the same client instance
//!
//! ### Important Notes
//! - This is a **blocking/synchronous** client - threads will sleep/block
//! - For async workloads, consider wrapping calls in `tokio::task::spawn_blocking`
//! - Timeouts are controlled by the underlying `reqwest` client (default: 30s connect, no read timeout)
//!
//! ## Example
//!
//! ```no_run
//! use throttled_json_rpc::jsonrpc_client;
//!
//! jsonrpc_client!(pub struct MyRpcClient {
//!     single:
//!         /// Get block hash by height
//!         pub fn getblockhash(&self, height: u64) -> Result<String>;
//!     enum:
//! });
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = MyRpcClient::new(
//!     "http://localhost:8332".to_string(),
//!     Some("rpcuser".to_string()),
//!     Some("rpcpass".to_string()),
//!     5,    // max 5 concurrent requests
//!     10,   // max 10 requests per second
//!     0,    // no batching (0 = disabled)
//! );
//!
//! let block_hash = client.getblockhash(100)?;
//! println!("Block hash: {}", block_hash);
//! # Ok(())
//! # }
//! ```

use thiserror::Error;

/// Error types for JSON-RPC operations
#[derive(Error, Debug)]
pub enum RpcError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON deserialization failed
    #[error("JSON deserialization failed: {source}\nBody: {body}")]
    JsonError {
        source: serde_json::Error,
        body: String,
    },

    /// RPC server returned an error
    #[error("RPC error: {error:?}")]
    RpcError { error: serde_json::Value },

    /// Response missing required ID field
    #[error("Response missing ID field")]
    MissingId,

    /// Response missing in batch result
    #[error("Missing response in batch result")]
    MissingResponse,

    /// RPC returned null result
    #[error("RPC returned null result")]
    NullResponse,

    /// Wrong enum variant for response
    #[error("Wrong variant of {enum_name}: expected {expected}")]
    WrongVariant {
        enum_name: &'static str,
        expected: &'static str,
    },

    /// Cannot deserialize to any enum variant
    #[error("Cannot deserialize to any variant of {enum_name}:\n{body}")]
    CannotDeserialize {
        enum_name: &'static str,
        body: String,
    },
}

#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_expansion() {
        jsonrpc_client!(pub struct TestClient {
            single:
                pub fn test_method(&self, arg: u64) -> Result<String>;
            enum:
                pub fn poly_method(&self) -> Result<A(String)|B(u64)>;
        });

        // Test that the macro expands without errors
        let _client = TestClient::new("http://localhost:8332".to_string(), None, None, 0, 0, 0);
    }
}
