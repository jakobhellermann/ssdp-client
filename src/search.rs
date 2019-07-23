use crate::{
    error::{Error, ParseSearchTargetError},
    parse_headers,
};
use futures_timer::FutureExt;
use romio::UdpSocket;
use std::{fmt, io::ErrorKind::TimedOut, net::SocketAddr};

#[derive(Eq, PartialEq)]
/// Specify what SSDP control points to search for
pub enum SearchTarget {
    /// Search for all devices and services.
    All,
    /// Search for root devices only.
    RootDevice,
    /// Search for a particular device. device-UUID specified by UPnP vendor.
    UUID(String),
    /// e.g. schemas-upnp-org:device:ZonePlayer:1
    DeviceURN(String, String, u32),
    /// e.g. schemas-sonos-com:service:Queue:1
    ServiceURN(String, String, u32),
}
impl fmt::Display for SearchTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchTarget::All => "ssdp:all".fmt(f),
            SearchTarget::RootDevice => "upnp:rootdevice".fmt(f),
            SearchTarget::UUID(uuid) => write!(f, "uuid:{}", uuid),
            SearchTarget::DeviceURN(domain, type_, ver) => {
                write!(f, "urn:{}:device:{}:{}", domain, type_, ver)
            }
            SearchTarget::ServiceURN(domain, type_, ver) => {
                write!(f, "urn:{}:service:{}:{}", domain, type_, ver)
            }
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
            return Ok(SearchTarget::UUID(
                str.trim_start_matches("uuid:").to_string(),
            ));
        }

        let mut iter = str.split(':');
        if iter.next() != Some("urn") {
            return Err(ParseSearchTargetError::new());
        }

        let domain = iter
            .next()
            .ok_or_else(ParseSearchTargetError::new)?
            .to_string();
        let device_or_service = iter.next().ok_or_else(ParseSearchTargetError::new)?;
        let type_ = iter
            .next()
            .ok_or_else(ParseSearchTargetError::new)?
            .to_string();
        let version = iter
            .next()
            .ok_or_else(ParseSearchTargetError::new)?
            .parse::<u32>()
            .map_err(|_| ParseSearchTargetError::new())?;

        if iter.next() != None {
            return Err(ParseSearchTargetError::new());
        }

        match device_or_service {
            "device" => Ok(SearchTarget::DeviceURN(domain, type_, version)),
            "service" => Ok(SearchTarget::ServiceURN(domain, type_, version)),
            _ => Err(ParseSearchTargetError::new()),
        }
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
/// Control Points will wait a random amount of time between 0 and mx seconds before responing to avoid flooding the requester with responses.
/// Therefore, the timeout should be at least mx seconds.
pub async fn search(
    search_target: SearchTarget,
    timeout: std::time::Duration,
    mx: usize,
) -> Result<Vec<SearchResponse>, Error> {
    let bind_addr: SocketAddr = ([0, 0, 0, 0], 0).into();
    let broadcast_address: SocketAddr = ([239, 255, 255, 250], 1900).into();

    let mut socket = UdpSocket::bind(&bind_addr)?;

    let msg = format!(
        "M-SEARCH * HTTP/1.1\r
Host:239.255.255.250:1900\r
Man:\"ssdp:discover\"\r
ST: {}\r
MX: {}\r\n\r\n",
        search_target, mx
    );
    socket.send_to(msg.as_bytes(), &broadcast_address).await?;

    let mut responses = Vec::new();

    loop {
        let mut buf = [0u8; 1024];
        let text = match socket.recv_from(&mut buf).timeout(timeout).await {
            Ok((read, _)) if read == 1024 => unimplemented!(), // TODO
            Ok((read, _)) => std::str::from_utf8(&buf[..read])?,
            Err(e) if e.kind() == TimedOut => break Ok(responses),
            Err(e) => return Err(e.into()),
        };

        let (location, st, usn) = parse_headers!(text => location, st, usn);

        responses.push(SearchResponse {
            location: location.to_string(),
            st: st.parse()?,
            usn: usn.to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::SearchTarget;

    #[test]
    fn parse_search_target() {
        assert_eq!("ssdp:all".parse(), Ok(SearchTarget::All));
        assert_eq!("upnp:rootdevice".parse(), Ok(SearchTarget::RootDevice));

        assert_eq!(
            "uuid:some-uuid".parse(),
            Ok(SearchTarget::UUID("some-uuid".to_string()))
        );

        assert_eq!(
            "urn:schemas-upnp-org:device:ZonePlayer:1".parse(),
            Ok(SearchTarget::DeviceURN(
                "schemas-upnp-org".into(),
                "ZonePlayer".into(),
                1
            ))
        );
        assert_eq!(
            "urn:schemas-sonos-com:service:Queue:2".parse(),
            Ok(SearchTarget::ServiceURN(
                "schemas-sonos-com".into(),
                "Queue".into(),
                2
            ))
        );
    }
}
