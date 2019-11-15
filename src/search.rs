use crate::{Error, SearchTarget};

use async_std::net::UdpSocket;
use async_std::prelude::*;

use std::net::SocketAddr;
use std::time::Duration;

pub(crate) const INSUFFICIENT_BUFFER_MSG: &str = "buffer size too small, udp packets lost";

#[derive(Debug)]
/// Response given by ssdp control point
pub struct SearchResponse {
    pub(crate) location: String,
    pub(crate) st: SearchTarget,
    pub(crate) usn: String,
    pub(crate) server: String,
}

impl SearchResponse {
    /// URL of the control point
    pub fn location(&self) -> &str {
        &self.location
    }
    /// search target returned by the control point
    pub fn search_target(&self) -> &SearchTarget {
        &self.st
    }
    /// Unique Service Name
    pub fn usn(&self) -> &str {
        &self.usn
    }
    /// Server (user agent)
    pub fn server(&self) -> &str {
        &self.server
    }
}

/// Search for SSDP control points within a network.
/// Control Points will wait a random amount of time between 0 and mx seconds before responing to avoid flooding the requester with responses.
/// Therefore, the timeout should be at least mx seconds.
pub async fn search(
    search_target: &SearchTarget,
    timeout: Duration,
    mx: usize,
) -> Result<impl Stream<Item = Result<SearchResponse, Error>>, Error> {
    let bind_addr: SocketAddr = ([0, 0, 0, 0], 0).into();
    let broadcast_address: SocketAddr = ([239, 255, 255, 250], 1900).into();

    let socket = UdpSocket::bind(&bind_addr).await?;

    let msg = format!(
        "M-SEARCH * HTTP/1.1\r
Host:239.255.255.250:1900\r
Man:\"ssdp:discover\"\r
ST: {}\r
MX: {}\r\n\r\n",
        search_target, mx
    );
    socket.send_to(msg.as_bytes(), &broadcast_address).await?;

    #[cfg(not(feature = "nightly"))]
    return search_socket_stream(socket, timeout).await;
    #[cfg(feature = "nightly")]
    return Ok(search_socket_stream(socket, timeout));
}

#[cfg(not(feature = "nightly"))]
async fn search_socket_stream(
    socket: UdpSocket,
    timeout: Duration,
) -> Result<impl Stream<Item = Result<SearchResponse, Error>>, Error> {
    use async_std::io;
    use std::io::ErrorKind::TimedOut;

    let mut responses = Vec::new();
    loop {
        let mut buf = [0u8; 2048];
        let text = match io::timeout(timeout, socket.recv(&mut buf)).await {
            Ok(read) if read == 2048 => panic!(INSUFFICIENT_BUFFER_MSG),
            Ok(read) => std::str::from_utf8(&buf[..read])?,
            Err(e) if e.kind() == TimedOut => break,
            Err(e) => return Err(e.into()),
        };

        let headers = crate::parse_headers(text)?;

        let mut location = None;
        let mut st = None;
        let mut usn = None;
        let mut server = None;

        for (header, value) in headers {
            if header.eq_ignore_ascii_case("location") {
                location = Some(value);
            } else if header.eq_ignore_ascii_case("st") {
                st = Some(value);
            } else if header.eq_ignore_ascii_case("usn") {
                usn = Some(value);
            } else if header.eq_ignore_ascii_case("server") {
                server = Some(value);
            }
        }

        let location = location
            .ok_or(Error::MissingHeader("location"))?
            .to_string();
        let st = st.ok_or(Error::MissingHeader("st"))?.parse()?;
        let usn = usn.ok_or(Error::MissingHeader("urn"))?.to_string();
        let server = server.ok_or(Error::MissingHeader("server"))?.to_string();

        responses.push(SearchResponse {
            location,
            st,
            usn,
            server,
        });
    }

    Ok(async_std::stream::from_iter(responses.into_iter().map(Ok)))
}

#[cfg(feature = "nightly")]
use crate::search_unstable::search_socket_stream;
