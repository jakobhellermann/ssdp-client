#![feature(stmt_expr_attributes, proc_macro_hygiene)]

use async_std::{io, net::TcpListener, task};
use futures::prelude::*;
use futures_async_stream::for_await;

fn main() -> Result<(), ssdp_client::Error> {
    task::block_on(subscribe())
}

async fn subscribe() -> Result<(), ssdp_client::Error> {
    let control_point: std::net::SocketAddr = ([192, 168, 2, 49], 1400).into();
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

    let addr: std::net::SocketAddr = ([192, 168, 2, 91], 7878).into();
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);
    #[for_await]
    for stream in listener.incoming() {
        stream?.copy_into(&mut io::stdout()).await?;
        println!();
    }

    Ok(())
}
