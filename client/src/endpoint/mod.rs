//! This module contains the various end point definitions for stellar's horizon
//! API server. An endpoint is a struct that implement the `IntoRequest` trait
//! and can be given to a client for fetching into a response.
//!
//! # Example
//! ```
//! use stellar_client::sync::Client;
//! use stellar_client::endpoint::transaction;
//!
//! // Instantiate a client connected to the horizon test network
//! let client = Client::horizon_test().unwrap();
//!
//! // Create a struct that represents the "all" transactions endpoint
//! let txns = transaction::All::default();
//!
//! // Give the endpoint to the client and receive back a `Records` struct
//! // that provides the resulting set of transactions
//! let all_txns = client.request(txns).unwrap();
//! ```
use error::Result;
use serde::de::DeserializeOwned;
use http;

pub mod account;
pub mod asset;
pub mod effect;
pub mod ledger;
pub mod operation;
pub mod payment;
pub mod transaction;

mod cursor;
mod limit;
mod records;
mod flat_records;
mod order;

pub use self::cursor::Cursor;
pub use self::limit::Limit;
pub use self::records::Records;
pub use self::flat_records::FlatRecords;
pub use self::order::{Direction, Order};

/// Represents the body of a request to an IntoRequest.
#[derive(Debug)]
pub enum Body {
    /// Declares that the endpoint does not have a body.
    None,
}

/// Declares the definition of a stellar endpoint and the return type.
pub trait IntoRequest {
    /// The deserializable type that is expected to come back from the stellar server.
    type Response: DeserializeOwned;
    /// The request body to be sent to stellar. Generally this is just a `()` unit.

    /// Converts the implementing struct into an http request.
    fn into_request(self, host: &str) -> Result<http::Request<Body>>;
}
