use futures::prelude::*;
use ssdp_client::URN;
use std::time::Duration;
use std::net::{SocketAddrV6, Ipv6Addr};

#[tokio::main]
async fn main() -> Result<(), ssdp_client::Error> {
    let search_target = URN::device("schemas-upnp-org", "ZonePlayer", 1).into();
    let timeout = Duration::from_secs(10);
    // ipv4
    // let bind_addr = ([192, 168, 1, 10], 1900).into();
    // ipv6
    let bind_addr = SocketAddrV6::new(
        Ipv6Addr::new(0xfe80, 0, 0, 0, 0x64a3, 0x40ff, 0xfe0b, 0x77a3),
        1900, // port
        0,    // stream info: 0
        4     // iface index: 4
    ).into();
    let mut responses = ssdp_client::search(&search_target, timeout, 2, None, bind_addr).await.expect("asjdlksaj");

    while let Some(response) = responses.next().await {
        let response = response?;
        println!("- {}", response.search_target());
        println!("  - location: {}", response.location());
        println!("  - usn: {}", response.usn());
        println!("  - server: {}", response.server());
        println!(
            "  - properties: {:?}",
            response.extra_header("PROPERTIES.TEST")
        );
    }

    Ok(())
}
