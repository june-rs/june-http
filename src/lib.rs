#![forbid(future_incompatible)]

pub mod request;
pub mod response;
pub mod routers;
pub mod server;

pub use request::{Method, Request};
pub use response::{Response, Status};
pub use server::{Server, Service};

pub use http::header;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
