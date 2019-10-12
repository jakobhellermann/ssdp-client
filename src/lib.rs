#![feature(generators, proc_macro_hygiene)]
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
//! ```rust,norun
//! # #![feature(proc_macro_hygiene, stmt_expr_attributes)]
//! # async fn f() -> Result<(), ssdp_client::Error> {
//! use std::time::Duration;
//! use ssdp_client::SearchTarget;
//! use futures_async_stream::for_await;
//!
//! let search_target = SearchTarget::RootDevice;
//! let responses = ssdp_client::search(&search_target, Duration::from_secs(3), 2).await?;
//!
//! #[for_await]
//! for response in responses {
//!     println!("{:?}", response);
//! }
//! # return Ok(());
//! # }
//! ```

/// SSDP Error types
pub mod error;
mod search;
mod search_target;

pub use error::Error;
pub use search::search;
pub use search_target::{SearchTarget, URN};
