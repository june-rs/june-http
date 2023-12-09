use std::io::{Error, ErrorKind, Result as IOResult};

use crate::response::encoding::ContentEncoding;

use ahash::AHashMap;
use bytes::Bytes;
use httparse::{Request as HTTParseRequest, EMPTY_HEADER};
use serde_querystring::from_str;
pub use serde_querystring::ParseMode as QueryMode;

pub struct Request<'a> {
    pub raw: Bytes,
    pub method: Method,
    pub path: Path<'a>,
    pub body: &'a [u8],
}

impl<'a> Request<'a> {
    pub fn parse() -> Option<Request<'a>> {
        None
    }
}

// METHOD

use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, AsRefStr, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

// PATH

type Queries<'a> = AHashMap<&'a str, Vec<&'a str>>;

pub struct Path<'a> {
    pub location: &'a str,
    pub query_string: &'a str,
    pub queries: Queries<'a>,
}

impl<'a> Path<'a> {
    pub fn new(raw: &'a str, query_mode: QueryMode) -> IOResult<Path<'a>> {
        Ok(match raw.split_once('?') {
            Some((location, query_string)) => {
                let queries = match from_str(query_string, query_mode) {
                    Ok(q) => q,
                    Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
                };

                Path {
                    location,
                    query_string,
                    queries,
                }
            }
            None => Path {
                location: raw,
                query_string: "",
                queries: Queries::new(),
            },
        })
    }

    #[inline]
    pub fn get_query(&'a self, key: &str) -> IOResult<&'a Vec<&'a str>> {
        match self.queries.get(key) {
            Some(value) => Ok(value),
            None => Err(Error::new(ErrorKind::NotFound, key)),
        }
    }

    #[inline]
    pub fn get_query_single(&'a self, key: &str) -> IOResult<&'a str> {
        Ok(self.get_query(key)?[0])
    }
}
