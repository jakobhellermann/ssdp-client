use crate::{parse_headers, Error};
use futures_timer::FutureExt;
use romio::UdpSocket;
use std::{io::ErrorKind::TimedOut, net::SocketAddr};

mod search_target;
pub use search_target::*;

#[derive(Debug)]
/// Response given by ssdp control point
pub struct SearchResponse<'s> {
    location: String,
    st: SearchTarget<'s>,
    usn: String,
}

impl SearchResponse<'_> {
    /// URL of the control point
    pub fn location(&self) -> &String {
        &self.location
    }
    /// search target returned by the control point
    pub fn search_target(&self) -> &SearchTarget<'_> {
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
    search_target: SearchTarget<'_>,
    timeout: std::time::Duration,
    mx: usize,
) -> Result<Vec<SearchResponse<'_>>, Error> {
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
        let mut buf = [0u8; 2048];
        let text = match socket.recv_from(&mut buf).timeout(timeout).await {
            Ok((read, _)) if read == 2048 => {
                handle_insufficient_buffer_size();
                continue;
            }
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

const INSUFFICIENT_BUFFER_MSG: &str = "buffer size too small, udp packets lost";
#[cfg(debug_assertions)]
fn handle_insufficient_buffer_size() {
    panic!(INSUFFICIENT_BUFFER_MSG);
}
#[cfg(not(debug_assertions))]
fn handle_insufficient_buffer_size() {
    log::warn!(INSUFFICIENT_BUFFER_MSG);
}
