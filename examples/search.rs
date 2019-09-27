#![feature(proc_macro_hygiene, stmt_expr_attributes)]

use async_std::task;
use futures_async_stream::for_await;
use ssdp_client::search::SearchTarget;
use std::time::Duration;

fn main() -> Result<(), ssdp_client::Error> {
    task::block_on(search())
}

async fn search() -> Result<(), ssdp_client::Error> {
    //let search_target = SearchTarget::RootDevice;
    let search_target: SearchTarget = "urn:schemas-upnp-org:device:ZonePlayer:1".parse().unwrap();
    let timeout = Duration::from_secs(3);
    let stream = ssdp_client::search(search_target, timeout, 2).await?;

    #[for_await]
    for response in stream {
        let response = response?;
        println!("- {}", response.search_target());
        println!("  - location: {}", response.location());
        println!("  - usn: {}", response.usn());
        println!("  - server: {}", response.server());
    }

    Ok(())
}
