use futures::prelude::*;
use romio::TcpStream;
use std::io;
use std::net::SocketAddr;

/// Subscribe to a service using a callback.
/// `addr` is the address of the control point,
/// `endpoint` the control url path for your service, e.g. "/MediaRenderer/AVTransport/Event"
pub async fn subscribe<'a>(
    addr: &'a SocketAddr,
    endpoint: &'a str,
    callback: &'a str,
    timeout: u32,
) -> Result<(), io::Error> {
    let msg = format!(
        "SUBSCRIBE {} HTTP/1.1\r
Host: {}\r
CALLBACK: <{}>\r
NT: upnp:event\r
TIMEOUT: Second-{}\r\n\r\n",
        endpoint,
        addr.to_string(),
        callback,
        timeout
    );
    println!("{}", msg);

    let mut stdout = futures::io::AllowStdIo::new(io::stdout());
    let stream = TcpStream::connect(addr).await?;
    let (mut reader, mut writer) = stream.split();
    writer.write_all(msg.as_bytes()).await?;
    reader.copy_into(&mut stdout).await?;
    println!();

    Ok(())
}
