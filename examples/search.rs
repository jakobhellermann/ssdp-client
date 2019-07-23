#![feature(async_await)]

use ssdp_client::search::SearchTarget;
use std::time::Duration;

#[runtime::main]
async fn main() -> Result<(), ssdp_client::Error> {
    let search_target = SearchTarget::RootDevice;
    let timeout = Duration::from_secs(3);
    let responses = ssdp_client::search(search_target, timeout, 2).await?;

    for response in responses {
        println!("{:?}", response);
    }

    Ok(())
}
