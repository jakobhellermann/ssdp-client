use crate::error::{ParseSearchTargetError, SSDPError};
use futures_timer::FutureExt;
use romio::UdpSocket;
use std::collections::HashMap;
use std::fmt;
use std::io::ErrorKind::TimedOut;
use std::net::SocketAddr;

#[derive(Eq, PartialEq)]
/// Specify what SSDP control points to search for
pub enum SearchTarget {
    /// Search for all devices and services.
    All,
    /// Search for root devices only.
    RootDevice,
    /// Search for a particular device. device-UUID specified by UPnP vendor.
    UUID(String),
    /// Match URN.
    /// e.g. schemas-sonos-com:service:Queue:1
    URN(String),
}
impl fmt::Display for SearchTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchTarget::All => "ssdp:all".fmt(f),
            SearchTarget::RootDevice => "upnp:rootdevice".fmt(f),
            SearchTarget::UUID(uuid) => write!(f, "uuid:").and(write!(f, "{}", uuid)),
            SearchTarget::URN(urn) => write!(f, "urn:").and(write!(f, "{}", urn)),
        }
    }
}
impl fmt::Debug for SearchTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
impl std::str::FromStr for SearchTarget {
    type Err = ParseSearchTargetError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str == "ssdp:all" {
            return Ok(SearchTarget::All);
        }
        if str == "upnp:rootdevice" {
            return Ok(SearchTarget::RootDevice);
        }
        if str.starts_with("uuid:") {
            return Ok(SearchTarget::UUID(str[4..].to_string()));
        }
        if str.starts_with("urn:") {
            return Ok(SearchTarget::URN(str[3..].to_string()));
        }

        Err(ParseSearchTargetError::new())
    }
}

#[derive(Debug)]
/// Response given by ssdp control point
pub struct SearchResponse {
    location: String,
    st: SearchTarget,
    usn: String,
}
impl SearchResponse {
    fn new(location: String, st: SearchTarget, usn: String) -> Self {
        Self { location, st, usn }
    }
    /// URL of the control point
    pub fn location(&self) -> &String {
        &self.location
    }
    /// search target returned by the control point
    pub fn search_target(&self) -> &SearchTarget {
        &self.st
    }
    /// Unique Service Name
    pub fn usn(&self) -> &String {
        &self.usn
    }
}

/// Search for SSDP control points within a network.
pub async fn search(
    search_target: SearchTarget,
    timeout: std::time::Duration,
) -> Result<Vec<SearchResponse>, SSDPError> {
    let bind_addr: SocketAddr = ([0, 0, 0, 0], 0).into();
    let broadcast_address: SocketAddr = ([239, 255, 255, 250], 1900).into();

    let mut socket = UdpSocket::bind(&bind_addr)?;

    let msg = format!(
        "M-SEARCH * HTTP/1.1\r
Host:239.255.255.250:1900\r
Man:\"ssdp:discover\"\r
ST: {}\r
MX: 3\r\n\r\n",
        search_target
    );
    socket.send_to(msg.as_bytes(), &broadcast_address).await?;

    let mut responses = Vec::new();

    loop {
        let mut buf = [0u8; 1000];
        let text = match socket.recv_from(&mut buf).timeout(timeout).await {
            Ok((read, _)) => std::str::from_utf8(&buf[..read])?,
            Err(e) if e.kind() == TimedOut => break Ok(responses),
            Err(e) => return Err(e.into()),
        };

        let headers: HashMap<&str, &str> = text
            .split("\r\n")
            .skip(1)
            .filter_map(|l| {
                let mut split = l.splitn(2, ':');
                match (split.next(), split.next()) {
                    (Some(header), Some(value)) => Some((header, value.trim())),
                    _ => None,
                }
            })
            .collect();

        if let Some(location) = headers.get("LOCATION") {
            if let Some(st) = headers.get("ST") {
                if let Some(usn) = headers.get("USN") {
                    responses.push(SearchResponse::new(
                        location.to_string(),
                        st.parse()?,
                        usn.to_string(),
                    ));
                } else {
                    return Err(SSDPError::MissingHeader("USN"));
                }
            } else {
                return Err(SSDPError::MissingHeader("ST"));
            }
        } else {
            return Err(SSDPError::MissingHeader("LOCATION"));
        }
    }
}
