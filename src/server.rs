use std::io::Result;
use std::net::ToSocketAddrs;

use crate::{Request, Response};

use may::coroutine::JoinHandle;
use may::go;
use may::net::TcpListener;

// VERSION - does nothing for now... HTTP/2.0 support at some point ;3

pub enum Version {
    V1_1,
}

// SERVICE

pub trait Service {
    fn call(&self, req: &Request, res: &mut Response) -> Result<()>;
}

// SERVER

pub struct Server<T: Service>(pub T);

impl<T: Service + Clone + Send + Sync + 'static> Server<T> {
    pub fn start<A: ToSocketAddrs>(self, addr: A) -> Result<JoinHandle<()>> {
        let listener = TcpListener::bind(addr)?;
        let service = self.0;

        Ok(go!(move || {
            for stream in listener.incoming() {
                let service = service.clone();
                go!(move || {});
            }
        }))
    }
}
