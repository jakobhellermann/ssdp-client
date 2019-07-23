#![feature(async_await)]

use ssdp::search::SearchTarget;
use std::time::Duration;

#[runtime::main]
async fn main() -> Result<(), ssdp::Error> {
    let search_target = SearchTarget::RootDevice;
    let timeout = Duration::from_secs(3);
    let responses = ssdp::search(search_target, timeout, 2).await?;

    for response in responses {
        println!("{:?}", response);
    }

    Ok(())
}
