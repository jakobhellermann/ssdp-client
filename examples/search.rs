#![feature(async_await, proc_macro_hygiene, stmt_expr_attributes)]

use futures_async_stream::for_await;
use ssdp_client::search::SearchTarget;
use std::time::Duration;

#[runtime::main]
async fn main() -> Result<(), ssdp_client::Error> {
    let search_target = SearchTarget::RootDevice;
    let timeout = Duration::from_secs(3);
    let stream = ssdp_client::search(search_target, timeout, 2).await?;

    #[for_await]
    for response in stream {
        println!("{:?}", response);
    }

    Ok(())
}
