[![Build Status](https://travis-ci.com/jjakobh/ssdp-client.svg?branch=master)](https://travis-ci.com/jjakobh/ssdp-client)
[![Actions Status](https://github.com/jjakobh/ssdp-client/workflows/CI/badge.svg)](https://github.com/jjakobh/ssdp-client/actions)
![GitHub last commit](https://img.shields.io/github/last-commit/jjakobh/ssdp-client.svg)
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
use std::time::Duration;
use ssdp_client::SearchTarget;

let search_target = SearchTarget::RootDevice;
let responses = ssdp_client::search(&search_target, Duration::from_secs(3), 2).await?;

for response in responses {
    println!("{:?}", response);
}
```

# Features:
The `unstable-stream` feature makes `ssdp-client::search` return a `Stream` of `Result<SearchResponse, Error>` instead of a `Vec<_>`.
This currently only works on nightly due to the `futures-async-stream` dependency.
It also pulls in `syn` and `quote` expect compile times to take longer.

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
