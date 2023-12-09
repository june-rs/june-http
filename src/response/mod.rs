use std::fmt;

use http::header::{HeaderMap, HeaderValue};
use bytes::Bytes;

// MODULES

pub mod encoding;

// STATUS

pub struct Status<'a>(pub u16, pub &'a str);

#[cfg(unix)]
include!(concat!(env!("OUT_DIR"), "/statuses.rs"));
#[cfg(windows)]
include!(concat!(env!("OUT_DIR"), "\\statuses.rs"));

impl<'a> fmt::Display for Status<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

// HEADERS

const SERVER: &str = const_format::formatcp!("june/{} ({})", crate::VERSION, std::env::consts::FAMILY);

// ENCODINGS

use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, AsRefStr, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum TransferEncoding {
    Chunked,
    Compress,
    Deflate,
    Gzip,
}

// RESPONSE

pub struct Response<'a> {
    version: &'a str,

    pub headers: HeaderMap<HeaderValue>,
    pub status: Status<'a>,
    pub body: Bytes,
}

impl<'a> Response<'a> {
    pub fn encode(&self) -> Bytes {
        let start = format!(
            "\
            HTTP/{} {}\r\n\
            Server: {}\r\n\
            Content-Length: {}\r\n\
            ",
            self.version, self.status,
            SERVER,
            self.body.len(),
        );
    }

}

impl<'a> fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
            HTTP/{} {}\r\n\
            Server: {}\r\n\
            Content-Length: {}\r\n\
            ",
            self.version, self.status,
            SERVER,
            self.body.len(),
        )?;

        Ok(())
    }
}

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        Self {
            version: "1.1",
            status: Status::OK,
            body: Bytes::new(),
        }
    }
}
