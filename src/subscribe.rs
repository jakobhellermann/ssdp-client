use crate::{parse_headers, Error};
use futures::prelude::*;
use romio::TcpStream;
use std::net::SocketAddr;

/// Response given by Control Point after SUBSCRIBE http Request
#[derive(Debug)]
pub struct SubscribeResponse {
    sid: String,
    timeout: u64,
    server: String,
}

impl SubscribeResponse {
    /// unique Subscription Identifier
    pub fn sid(&self) -> &str {
        &self.sid
    }
    /// timeout for subscription
    pub fn timeout(&self) -> u64 {
        self.timeout
    }
    /// basically a user-agent
    pub fn server(&self) -> &str {
        &self.server
    }
}

/// Subscribe to a service using a callback.
/// `addr` is the address of the control point,
/// `endpoint` the control url path for your service, e.g. "/MediaRenderer/AVTransport/Event"
pub async fn subscribe(
    addr: &SocketAddr,
    endpoint: &str,
    callback: &str,
    timeout: u32,
) -> Result<SubscribeResponse, Error> {
    let msg = format!(
        "SUBSCRIBE {} HTTP/1.1\r
Host: {}\r
CALLBACK: <{}>\r
NT: upnp:event\r
TIMEOUT: Second-{}\r\n\r\n",
        // STATEVAR: csv of StateVariables
        endpoint,
        addr.to_string(),
        callback,
        timeout
    );

    let stream = TcpStream::connect(addr).await?;
    let (mut reader, mut writer) = stream.split();
    writer.write_all(msg.as_bytes()).await?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;

    let response = String::from_utf8(buf).map_err(|e| e.utf8_error())?;

    let (sid, timeout, server) = parse_headers!(response.as_ref() => sid, timeout, server)?;

    Ok(SubscribeResponse {
        sid: sid.to_string(),
        timeout: parse_timeout(timeout).map_err(|_| Error::InvalidHeader("TIMEOUT"))?,
        server: server.to_string(),
    })
}

fn parse_timeout(timeout: &str) -> Result<u64, std::num::ParseIntError> {
    timeout.trim_start_matches("Second-").parse()
}
