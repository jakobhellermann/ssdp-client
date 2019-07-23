#![feature(async_await)]

use futures::io::AllowStdIo;
use futures::prelude::*;
use romio::TcpListener;
use std::io;

#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([192, 168, 2, 49], 1400).into();
    let endpoint = "/MediaRenderer/AVTransport/Event";
    let callback = "http://192.168.2.91:7878";
    let timeout = 50;
    let response = ssdp_client::subscribe(&addr, endpoint, callback, timeout).await?;
    println!(
        "SID {} from {} with {}",
        response.sid(),
        response.server(),
        response.timeout()
    );

    let mut listener = TcpListener::bind(&"192.168.2.91:7878".parse().unwrap())?;
    let mut incoming = listener.incoming();

    println!("Listening on 192.168.2.91:7878");
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let mut stdout = AllowStdIo::new(io::stdout());
        stream.copy_into(&mut stdout).await?;
        println!();
    }

    Ok(())
}
