use crate::Error;

use async_std::io;
use async_std::net::UdpSocket;

use std::io::ErrorKind::TimedOut;
use std::time::Duration;

use crate::search::{SearchResponse, INSUFFICIENT_BUFFER_MSG};

#[futures_async_stream::async_try_stream(ok = SearchResponse, error = Error)]
pub async fn search_socket_stream(socket: UdpSocket, timeout: Duration) {
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

        yield SearchResponse {
            location,
            st,
            usn,
            server,
        };
    }
}
