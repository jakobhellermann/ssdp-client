#![warn(
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations
)]

//! An asynchronous library for discovering, notifying and subscribing to devices and services on a network.
//!
//! SSDP stands for Simple Service Discovery Protocol and it is a protocol that
//! distributes messages across a local network for devices and services to
//! discover each other. SSDP can most commonly be found in devices that implement
//! `UPnP` as it is used as the discovery mechanism for that standard.
//!
//! # Example
//! ```rust,no_run
//! # async fn f() -> Result<(), ssdp_client::Error> {
//! use futures::prelude::*;
//! use std::time::Duration;
//! use ssdp_client::SearchTarget;
//!
//! let search_target = SearchTarget::RootDevice;
//! let mut responses = ssdp_client::search(&search_target, Duration::from_secs(3), 2, None).await?;
//!
//! while let Some(response) = responses.next().await {
//!     println!("{:?}", response?);
//! }
//! # return Ok(());
//! # }
//! ```

/// SSDP Error types
mod error;
mod search;
mod search_target;

pub use error::Error;
pub use search::{search, SearchResponse};
pub use search_target::{SearchTarget, URN};
