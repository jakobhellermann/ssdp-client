use crate::SSDPError;
use futures::prelude::*;
use romio::TcpStream;
use std::net::SocketAddr;

/// Response given by Control Point after SUBSCRIBE http Request
#[derive(Debug)]
pub struct SubscribeResponse {
    sid: String,
    timeout: String,
    server: String,
}

impl SubscribeResponse {
    /// unique subscription identifier
    pub fn sid(&self) -> &str {
        &self.sid
    }
    /// timeout for subscription
    pub fn timeout(&self) -> &str {
        &self.timeout
    }
    /// basically a user-agent
    pub fn server(&self) -> &str {
        &self.server
    }
}

/// Subscribe to a service using a callback.
/// `addr` is the address of the control point,
/// `endpoint` the control url path for your service, e.g. "/MediaRenderer/AVTransport/Event"
pub async fn subscribe<'a>(
    addr: &'a SocketAddr,
    endpoint: &'a str,
    callback: &'a str,
    timeout: u32,
) -> Result<SubscribeResponse, SSDPError> {
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

    let s = String::from_utf8(buf).map_err(|e| e.utf8_error())?;

    let mut response = s.split("\r\n");
    assert_eq!(response.next(), Some("HTTP/1.1 200 OK")); // TODO

    let mut sid: Option<&str> = None;
    let mut server: Option<&str> = None;
    let mut timeout: Option<&str> = None;

    for (header, value) in response.filter_map(|l| {
        let mut split = l.splitn(2, ':');
        match (split.next(), split.next()) {
            (Some(header), Some(value)) => Some((header, value.trim())),
            _ => None,
        }
    }) {
        if header.eq_ignore_ascii_case("server") {
            server = Some(value);
        } else if header.eq_ignore_ascii_case("sid") {
            sid = Some(value);
        } else if header.eq_ignore_ascii_case("timeout") {
            timeout = Some(value);
        }
    }

    Ok(SubscribeResponse {
        sid: sid.ok_or(SSDPError::MissingHeader("SID"))?.to_string(),
        timeout: timeout
            .ok_or(SSDPError::MissingHeader("TIMEOUT"))?
            .to_string(),
        server: server
            .ok_or(SSDPError::MissingHeader("SERVER"))?
            .to_string(),
    })
}
