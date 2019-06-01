#![feature(async_await)]

use futures::io::AllowStdIo;
use futures::{task::SpawnExt, AsyncReadExt, StreamExt};
use romio::TcpListener;
use ssdp::SSDPError;
use std::io;

#[runtime::main]
async fn main() -> Result<(), failure::Error> {
    let addr = ([192, 168, 2, 49], 1400).into();
    let endpoint = "/MediaRenderer/AVTransport/Event";
    let callback = "http://192.168.2.91:7878";
    let timeout = 50;
    ssdp::subscribe(&addr, endpoint, callback, timeout).await?;

    let mut listener = TcpListener::bind(&"192.168.2.91:7878".parse().unwrap())?;
    let mut incoming = listener.incoming();

    println!("Listening on 192.168.2.91:7878");
    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;

        let mut stdout = AllowStrIo::new(io::stdout());
        stream.copy_into(&mut stdout).await?;
        println();
    }

    Ok(())
}
