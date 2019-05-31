#![feature(async_await, bind_by_move_pattern_guards)]

pub mod error;
pub mod search;
pub mod subscribe;

pub use error::SSDPError;
pub use search::{search, SearchTarget};
pub use subscribe::subscribe;
