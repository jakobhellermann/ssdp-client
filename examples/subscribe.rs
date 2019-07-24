#![feature(async_await)]

use futures::io::AllowStdIo;
use futures::prelude::*;
use romio::TcpListener;
use std::io;

#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let control_point = ([192, 168, 2, 49], 1400).into();
    let response = ssdp_client::subscribe(
        &control_point,
        "/MediaRenderer/AVTransport/Event",
        "http://192.168.2.91:7878", // localhost:7878
        10,
    )
    .await?;

    println!(
        "SID {} from {} with {}",
        response.sid(),
        response.server(),
        response.timeout()
    );

    let addr = ([192, 168, 2, 91], 7878).into();
    let mut listener = TcpListener::bind(&addr)?;
    let mut incoming = listener.incoming();

    println!("Listening on {}", addr);
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let mut stdout = AllowStdIo::new(io::stdout());
        stream.copy_into(&mut stdout).await?;
        println!();
    }

    Ok(())
}
