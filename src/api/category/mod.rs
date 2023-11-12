#[cfg(feature = "ssr")]
pub(super) mod server;

mod data;
mod api;

pub use api::*;