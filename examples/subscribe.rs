#![feature(async_await)]

use futures::executor::ThreadPool;
use futures::io::AllowStdIo;
use futures::{task::SpawnExt, AsyncReadExt, StreamExt};
use romio::TcpListener;
use ssdp::SSDPError;
use std::io;

#[runtime::main]
async fn main() -> Result<(), failure::Error> {
    let addr = ([192, 168, 2, 49], 1400).into();
    println!("subscribe");
    ssdp::subscribe(
        &addr,
        "/MediaRenderer/AVTransport/Event",
        "http://192.168.2.91:7878",
        50,
    )
    .await
    .map_err(SSDPError::IO)?;

    let mut threadpool = ThreadPool::new().unwrap();

    let mut listener = TcpListener::bind(&"192.168.2.91:7878".parse().unwrap())?;
    let mut incoming = listener.incoming();

    println!("Listening on 192.168.2.91:7878");
    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        //let addr = stream.peer_addr()?;

        threadpool
            .spawn(async move {
                let mut stdout = AllowStdIo::new(io::stdout());
                stream.copy_into(&mut stdout).await.unwrap();
                println!();
            })
            .unwrap();
    }

    Ok(())
}
