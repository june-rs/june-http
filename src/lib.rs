#![forbid(future_incompatible)]

pub mod request;
pub mod response;
pub mod routers;
pub mod server;

pub use request::{Method, Request};
pub use response::{Response, Status};
pub use server::{Server, Service};

pub use http::header;

// USEFUL STUFF

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

// borrowed from nightly rust; [u8].as_ascii()
pub(crate) fn fastscii(bytes: &[u8]) -> &str {
    let ascii_ptr: *const [u8] = bytes;
    let str_ptr = ascii_ptr as *const str;
    // SAFETY: Each ASCII codepoint in UTF-8 is encoded as one single-byte
    // code unit having the same value as the ASCII byte.
    unsafe { &*str_ptr }
}