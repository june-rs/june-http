use std::io::Result;
use std::net::ToSocketAddrs;

use std::fmt;

use crate::{Request, Response};

use ahash::AHashSet;
use may::coroutine::JoinHandle;
use may::go;
use may::net::TcpListener;

// VERSION - does nothing for now... HTTP/2.0 support at some point ;3

#[derive(Hash, PartialEq, Eq)]
pub enum Version {
    HTTP1_0,
    HTTP1_1,
    HTTP2,
    HTTP3,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HTTP1_0 => write!(f, "HTTP/1.0"),
            Self::HTTP1_1 => write!(f, "HTTP/1.1"),
            Self::HTTP2 => write!(f, "HTTP/2.0"),
            Self::HTTP3 => write!(f, "HTTP/3.0"),
        }
    }
}

// SERVICE

pub trait Service {
    fn call(&self, req: &Request, res: &mut Response) -> Result<()>;
}

// SERVER

pub struct Server<T: Service> {
    versions: AHashSet<Version>,
    service: T,
}

impl<T: Service + Clone + Send + Sync + 'static> Server<T> {
    pub fn new(versions: Vec<Version>, service: T) -> Self {
        Self {
            versions: AHashSet::from_iter(versions.into_iter()),
            service,
        }
    }

    pub fn start<A: ToSocketAddrs>(self, addr: A) -> Result<JoinHandle<()>> {
        let listener = TcpListener::bind(addr)?;
        let service = self.service;

        Ok(go!(move || {
            for stream in listener.incoming() {
                let service = service.clone();
                go!(move || {});
            }
        }))
    }
}
