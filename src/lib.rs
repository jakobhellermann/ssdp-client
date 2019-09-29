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

/// SSDP Error types
pub mod error;
/// Methods and structs for dealing with searching devices
/// # Example
/// ```rust,norun
/// # #![feature(proc_macro_hygiene, stmt_expr_attributes)]
/// # async fn f() -> Result<(), ssdp_client::Error> {
/// # use std::time::Duration;
/// # use ssdp_client::SearchTarget;
/// use futures_async_stream::for_await;
///
/// let search_target = "urn:schemas-upnp-org:device:ZonePlayer:1".parse().unwrap();
/// // let search_target = SearchTarget::RootDevice;
/// let responses = ssdp_client::search(search_target, Duration::from_secs(3), 2).await?;
///
/// #[for_await]
/// for response in responses {
///     println!("{:?}", response);
/// }
/// # return Ok(());
/// # }
/// ```
pub mod search;
/// Methods and structs for dealing with subscribing to devices
/// # Example
/// ```rust,norun
/// # #![feature(stmt_expr_attributes, proc_macro_hygiene)]
/// # async fn subscribe() -> Result<(), ssdp_client::Error> {
/// let control_point = ([192, 168, 2, 49], 1400).into();
/// let response = ssdp_client::subscribe(
///     &control_point,
///     "/MediaRenderer/AVTransport/Event",
///     "http://192.168.2.91:7878", // localhost:7878
///     10,
/// )
/// .await?;
///
/// println!(
///     "SID {} from {} with {}",
///     response.sid(),
///     response.server(),
///     response.timeout()
/// );
/// # Ok(())
/// # }
/// ```
/// see full example at `examples` folder
pub mod subscribe;

pub use error::Error;
pub use search::{search, SearchTarget};
pub use subscribe::subscribe;

#[macro_export]
#[doc(hidden)]
macro_rules! parse_headers {
    ( $response:expr => $($header:ident),+ ) => { {
        parse_headers($response)
            .and_then(|headers| {
                $(let mut $header: Option<&str> = None;)*

                for (header, value) in headers {
                    $(if header.eq_ignore_ascii_case(stringify!($header)) {
                        $header = Some(value);
                    })else*
                }

                Ok(($($header.ok_or(crate::Error::MissingHeader(stringify!($header)))?),*))
            })
    } }
}

fn parse_headers(response: &str) -> Result<impl Iterator<Item = (&str, &str)>, crate::Error> {
    let mut response = response.split("\r\n");
    if let Some(status) = response.next() {
        let status = status.trim_start_matches("HTTP/1.1 ");
        let status_code = status
            .chars()
            .take_while(|x| x.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .map_err(|_| crate::Error::InvalidHTTP("status code is not a number"))?;
        if status_code != 200 {
            return Err(crate::Error::HTTPError(status_code));
        }
    } else {
        return Err(crate::Error::InvalidHTTP("http response is empty"));
    }

    Ok(response.filter_map(|l| {
        let mut split = l.splitn(2, ':');
        match (split.next(), split.next()) {
            (Some(header), Some(value)) => Some((header, value.trim())),
            _ => None,
        }
    }))
}
