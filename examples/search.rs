use async_std::prelude::*;
use ssdp_client::URN;
use std::time::Duration;

fn main() -> Result<(), ssdp_client::Error> {
    async_std::task::block_on(search())
}

async fn search() -> Result<(), ssdp_client::Error> {
    // let search_target = SearchTarget::RootDevice;
    let search_target = URN::device("schemas-upnp-org", "ZonePlayer", 1).into();
    let timeout = Duration::from_secs(3);
    let responses = ssdp_client::search(&search_target, timeout, 2).await?;
    pin_utils::pin_mut!(responses);

    while let Some(response) = responses.next().await {
        let response = response?;
        println!("- {}", response.search_target());
        println!("  - location: {}", response.location());
        println!("  - usn: {}", response.usn());
        println!("  - server: {}", response.server());
    }

    Ok(())
}
