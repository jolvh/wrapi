//! # Wrapi
//!
//! **Status**: 🚧 Work in Progress / Experimental
//!
//! Wrapi is a helper library for wrapping HTTP APIs using `reqwest` and `serde`.
//!
//! It provides a simple interface for making requests to an API and
//! deserializing the response.
//!
//! Requests are not tied to a client instance, allowing you to bring your own.
//!
//! ## Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use wrapi::http::Method;
//! use wrapi::request::Request;
//! use wrapi::reqwest::Client;
//!
//! #[derive(Serialize, Deserialize)]
//! struct NewUser {
//!     name: String,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct Id {
//!     id: u64,
//! }
//!
//! impl Request<Id> for NewUser {
//!     fn endpoint(&self) -> String {
//!         "user"
//!     }
//!
//!     fn method(&self) -> Method {
//!         Method::POST
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create HTTP client
//!     let client = Client::new();
//!
//!     // Create request payload
//!     let payload = NewUser {
//!         name: "John Doe".to_string(),
//!     };
//!
//!     // Send request
//!     let response = payload.send(&client, "<URI>").await.unwrap();
//!
//!     println!("{}", response.id);
//! }
//! ```

pub mod error;
pub mod parameters;
pub mod request;

// Re-exports
pub use http;
pub use reqwest;
