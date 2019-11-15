#![cfg_attr(feature = "nightly", feature(generators, proc_macro_hygiene))]
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
//! # async fn f() -> Result<(), ssdp_client::Error> {
//! # use async_std::prelude::*;
//! use ssdp_client::URN;
//! use std::time::Duration;
//!
//! let search_target = URN::device("schemas-upnp-org", "ZonePlayer", 1).into();
//! let timeout = Duration::from_secs(3);
//! let responses = ssdp_client::search(&search_target, timeout, 2).await?;
//! pin_utils::pin_mut!(responses);
//!
//! while let Some(response) = responses.next().await {
//!     println!("{:?}", response);
//! }
//! # return Ok(());
//! # }
//! ```
//!
//! # Features:
//! Without the `nightly` feature [`ssdp-client::search`](fn.search.html) is pretty slow
//! because it waits for all responses and the timeout before sending them all in one batch.
//! The feature currently only works on nightly due to the `futures-async-stream` dependency.
//! It also pulls in `syn` and `quote` expect compile times to take longer.

/// SSDP Error types
mod error;
mod search;
mod search_target;
#[cfg(feature = "nightly")]
mod search_unstable;

pub use error::Error;
pub use search::{search, SearchResponse};
pub use search_target::{SearchTarget, URN};

fn parse_headers(response: &str) -> Result<impl Iterator<Item = (&str, &str)>, Error> {
    let mut response = response.split("\r\n");
    let status_code = response
        .next()
        .ok_or(Error::InvalidHTTP("http response is empty"))?
        .trim_start_matches("HTTP/1.1 ")
        .chars()
        .take_while(|x| x.is_numeric())
        .collect::<String>()
        .parse::<u32>()
        .map_err(|_| Error::InvalidHTTP("status code is not a number"))?;

    if status_code != 200 {
        return Err(Error::HTTPError(status_code));
    }

    let iter = response.filter_map(|l| {
        let mut split = l.splitn(2, ':');
        match (split.next(), split.next()) {
            (Some(header), Some(value)) => Some((header, value.trim())),
            _ => None,
        }
    });

    Ok(iter)
}
