use std::fmt;

use crate::server::Version;

use http::header::{HeaderMap, HeaderValue};
use bytes::Bytes;

// MODULES

pub mod encoding;

use encoding::BodyEncoding;

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

// BODY

pub struct Body {
    pub encoding: BodyEncoding,
    pub data: Bytes,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            encoding: BodyEncoding::default(),
            data: Bytes::new(),
        }
    }
}

// RESPONSE

pub struct Response<'a> {
    pub headers: HeaderMap<HeaderValue>,
    pub status: Status<'a>,
    pub body: Body,
}

impl<'a> Response<'a> {
    pub fn encode_plaintext(&self, version: Version) -> Bytes {
        let start = format!(
            "\
            {} {}\r\n\
            Server: {}\r\n\
            Content-Length: {}\r\n\
            ",
            version, self.status,
            SERVER,
            self.body.data.len(),
        );

        start.into()
    }
}

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        Self {
            headers: HeaderMap::new(),
            status: Status::OK,
            body: Body::default(),
        }
    }
}
