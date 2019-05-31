use crate::SSDPError;
use futures_timer::FutureExt;
use romio::UdpSocket;
use std::collections::HashMap;
use std::fmt;
use std::io::ErrorKind::TimedOut;
use std::net::SocketAddr;

pub enum SearchTarget<'a> {
    All,
    RootDevice,
    UUID(&'a str),
    URN(&'a str),
}
impl Default for SearchTarget<'_> {
    fn default() -> Self {
        SearchTarget::All
    }
}
impl fmt::Display for SearchTarget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchTarget::All => "ssdp:all".fmt(f),
            SearchTarget::RootDevice => "upnp:rootdevice".fmt(f),
            SearchTarget::UUID(uuid) => write!(f, "uuid:").and(write!(f, "{}", uuid)),
            SearchTarget::URN(urn) => write!(f, "urn:").and(write!(f, "{}", urn)),
        }
    }
}

#[derive(Debug)]
pub struct SearchResponse {
    location: String,
    st: String,
    usn: String,
}
impl SearchResponse {
    fn new(location: String, st: String, usn: String) -> Self {
        Self { location, st, usn }
    }
    pub fn location(&self) -> &String {
        &self.location
    }
    pub fn st(&self) -> &String {
        &self.st
    }
}

pub async fn search(
    search_target: SearchTarget<'_>,
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
                        st.to_string(),
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
