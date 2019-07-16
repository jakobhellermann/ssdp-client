#![feature(async_await, bind_by_move_pattern_guards)]
#![deny(missing_docs, unsafe_code)]
#![warn(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations
)]

//! An asynchronous library for discovering, notifying and subscibing to devices and services on a network.
//!
//! SSDP stands for Simple Service Discovery Protocol and it is a protocol that
//! distributes messages across a local network for devices and services to
//! discover each other. SSDP can most commonly be found in devices that implement
//! `UPnP` as it is used as the discovery mechanism for that standard.

/// SSDP Error types
pub mod error;
/// Methods and structs for dealing with searching devices
pub mod search;
/// Methods and structs for dealing with subscribing to devices
pub mod subscribe;

pub use error::SSDPError;
pub use search::{search, SearchTarget};
pub use subscribe::subscribe;
