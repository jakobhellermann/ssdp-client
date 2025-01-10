![Build Status](https://github.com/jakobhellermann/ssdp-client/workflows/CI/badge.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/jakobhellermann/ssdp-client.svg)
[![Crates.io](https://img.shields.io/crates/v/ssdp-client.svg)](https://crates.io/crates/ssdp-client)

ssdp-client
=======
An asynchronous library for discovering, notifying and subscribing to devices and services on a network.

SSDP stands for Simple Service Discovery Protocol and it is a protocol that
distributes messages across a local network for devices and services to
discover each other. SSDP can most commonly be found in devices that implement
`UPnP` as it is used as the discovery mechanism for that standard.

**Technical Specification:**
http://upnp.org/specs/arch/UPnP-arch-DeviceArchitecture-v2.0.pdf

# Example usage:

```rust
use futures::prelude::*;
use std::time::Duration;
use ssdp_client::SearchTarget;

let search_target = SearchTarget::RootDevice;
let mut responses = ssdp_client::search(&search_target, Duration::from_secs(3), 2, None).await?;

while let Some(response) = responses.next().await {
    println!("{:?}", response?);
}
```

License
-------

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Contribution
------------

Please use [rustfmt](https://github.com/rust-lang/rustfmt) before any pull requests.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
