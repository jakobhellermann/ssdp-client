use futures::prelude::*;
use ssdp_client::URN;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), ssdp_client::Error> {
    let search_target = URN::device("schemas-upnp-org", "ZonePlayer", 1).into();
    let timeout = Duration::from_secs(3);
    let mut responses = ssdp_client::search(&search_target, timeout, 2, None).await?;

    while let Some(response) = responses.next().await {
        let response = response?;
        println!("- {}", response.search_target());
        println!("  - location: {}", response.location());
        println!("  - usn: {}", response.usn());
        println!("  - server: {}", response.server());
        println!("  - properties: {:?}", response.extra_header("PROPERTIES.TEST"));
    }

    Ok(())
}
