use crate::{Error, SearchTarget};

use futures_core::stream::Stream;
use genawaiter::sync::{Co, Gen};
use std::{collections::HashMap, net::SocketAddr, time::Duration};
use tokio::net::UdpSocket;

const INSUFFICIENT_BUFFER_MSG: &str = "buffer size too small, udp packets lost";
const DEFAULT_SEARCH_TTL: u32 = 2;

#[derive(Debug)]
/// Response given by ssdp control point
pub struct SearchResponse {
    location: String,
    st: SearchTarget,
    usn: String,
    server: String,
    extra_headers: HashMap<String, String>,
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
    /// Other Custom header
    pub fn extra_header(&self, key: &str) -> Option<&str> {
        self.extra_headers.get(key).map(|x| x.as_str())
    }
}

#[cfg(not(windows))]
async fn get_bind_addr() -> Result<SocketAddr, std::io::Error> {
    Ok(([0, 0, 0, 0], 0).into())
}

#[cfg(windows)]
async fn get_bind_addr() -> Result<SocketAddr, std::io::Error> {
    // Windows 10 is multihomed so that the address that is used for the broadcast send is not guaranteed to be your local ip address, it can be any of the virtual interfaces instead.
    // Thanks to @dheijl for figuring this out <3 (https://github.com/jakobhellermann/ssdp-client/issues/3#issuecomment-687098826)
    let any: SocketAddr = ([0, 0, 0, 0], 0).into();
    let socket = UdpSocket::bind(any).await?;
    let googledns: SocketAddr = ([8, 8, 8, 8], 80).into();
    socket.connect(googledns).await?;
    let bind_addr = socket.local_addr()?;

    Ok(bind_addr)
}

/// Search for SSDP control points within a network.
/// Control Points will wait a random amount of time between 0 and mx seconds before responing to avoid flooding the requester with responses.
/// Therefore, the timeout should be at least mx seconds.
pub async fn search(
    search_target: &SearchTarget,
    timeout: Duration,
    mx: usize,
    ttl: Option<u32>,
) -> Result<impl Stream<Item = Result<SearchResponse, Error>>, Error> {
    let bind_addr: SocketAddr = get_bind_addr().await?;
    let broadcast_address: SocketAddr = ([239, 255, 255, 250], 1900).into();

    let socket = UdpSocket::bind(&bind_addr).await?;
    socket
        .set_multicast_ttl_v4(ttl.unwrap_or(DEFAULT_SEARCH_TTL))
        .ok();

    let msg = format!(
        "M-SEARCH * HTTP/1.1\r
Host:239.255.255.250:1900\r
Man:\"ssdp:discover\"\r
ST: {}\r
MX: {}\r\n\r\n",
        search_target, mx
    );
    socket.send_to(msg.as_bytes(), &broadcast_address).await?;

    Ok(Gen::new(move |co| socket_stream(socket, timeout, co)))
}

macro_rules! yield_try {
    ( $co:expr => $expr:expr ) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                $co.yield_(Err(e.into())).await;
                continue;
            }
        }
    };
}

async fn socket_stream(
    socket: UdpSocket,
    timeout: Duration,
    co: Co<Result<SearchResponse, Error>>,
) {
    loop {
        let mut buf = [0u8; 2048];
        let text = match tokio::time::timeout(timeout, socket.recv(&mut buf)).await {
            Err(_) => break,
            Ok(res) => match res {
                Ok(read) if read == 2048 => {
                    log::warn!("{}", INSUFFICIENT_BUFFER_MSG);
                    continue;
                }
                Ok(read) => yield_try!(co => std::str::from_utf8(&buf[..read])),
                Err(e) => {
                    co.yield_(Err(e.into())).await;
                    continue;
                }
            },
        };

        let headers = yield_try!(co => parse_headers(text));

        let mut location = None;
        let mut st = None;
        let mut usn = None;
        let mut server = None;
        let mut extra_headers = HashMap::new();

        for (header, value) in headers {
            if header.eq_ignore_ascii_case("location") {
                location = Some(value);
            } else if header.eq_ignore_ascii_case("st") {
                st = Some(value);
            } else if header.eq_ignore_ascii_case("usn") {
                usn = Some(value);
            } else if header.eq_ignore_ascii_case("server") {
                server = Some(value);
            } else {
                extra_headers.insert(header.to_owned(), value.to_owned());
            }
        }

        let location = yield_try!(co => location
            .ok_or(Error::MissingHeader("location")))
        .to_string();
        let st = yield_try!(co => yield_try!(co => st.ok_or(Error::MissingHeader("st"))).parse::<SearchTarget>());
        let usn = yield_try!(co => usn.ok_or(Error::MissingHeader("urn"))).to_string();
        let server = yield_try!(co => server.ok_or(Error::MissingHeader("server"))).to_string();

        co.yield_(Ok(SearchResponse {
            location,
            st,
            usn,
            server,
            extra_headers,
        }))
        .await;
    }
}

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
