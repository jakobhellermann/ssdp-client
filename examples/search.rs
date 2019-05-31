#![feature(async_await)]

use ssdp::search::SearchTarget;
use ssdp::SSDPError;
use std::time::Duration;

#[runtime::main]
async fn main() -> Result<(), SSDPError> {
    let responses = ssdp::search(SearchTarget::RootDevice, Duration::from_secs(1)).await?;

    for response in responses {
        println!("{:?}", response);
    }

    Ok(())
}
