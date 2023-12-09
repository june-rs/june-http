use std::io::{Error, ErrorKind, Result};

use crate::{Method, Request, Response, Service};

use ahash::AHashMap;
use concat_string::concat_string;
use matchit::{Params, Router as MatchItRouter};

// ROUTERS - TRAIT

pub trait Router {
    type Service;

    fn insert(&mut self, method: Method, route: impl Into<String>, service: Self::Service);
    fn run(&self, req: &Request, res: &mut Response) -> Result<()>;
}

impl<T: Router> Service for T {
    #[inline]
    fn call(&self, req: &Request, res: &mut Response) -> Result<()> {
        self.run(req, res)
    }
}

// ROUTERS - SIMPLE

pub type SimpleService<'a> = &'a (dyn Fn(&Request, &mut Response) -> Result<()> + Send + Sync);

pub struct SimpleRouter<'a> {
    methods: AHashMap<Method, AHashMap<String, SimpleService<'a>>>,
}

impl<'a> SimpleRouter<'a> {
    #[inline]
    pub fn remove(&mut self, method: Method, route: impl Into<String>) {
        self.methods
            .entry(method)
            .or_insert_with(AHashMap::new)
            .remove(&route.into());
    }
}

impl<'a> Router for SimpleRouter<'a> {
    type Service = SimpleService<'a>;

    #[inline]
    fn insert(&mut self, method: Method, route: impl Into<String>, service: Self::Service) {
        self.methods
            .entry(method)
            .or_insert_with(AHashMap::new)
            .insert(route.into(), service);
    }

    fn run(&self, req: &Request, res: &mut Response) -> Result<()> {
        let routes = self.methods.get(&req.method).ok_or(Error::new(
            ErrorKind::NotFound,
            concat_string!("method: ", req.method.as_ref()),
        ))?;

        let service = routes.get(req.path.location).ok_or(Error::new(
            ErrorKind::NotFound,
            concat_string!(
                "method: ",
                req.method.as_ref(),
                ", locoation: ",
                req.path.location
            ),
        ))?;

        service(req, res)
    }
}

// ROUTERS - PARAMETER

pub type ParameterService<'a> =
    &'a (dyn Fn(&Request, &mut Response, Params) -> Result<()> + Send + Sync);

pub struct ParameterRouter<'a> {
    methods: AHashMap<Method, MatchItRouter<ParameterService<'a>>>,
}

impl<'a> Router for ParameterRouter<'a> {
    type Service = ParameterService<'a>;

    #[inline]
    fn insert(&mut self, method: Method, route: impl Into<String>, service: Self::Service) {
        self.methods
            .entry(method)
            .or_insert_with(MatchItRouter::new)
            .insert(route, service)
            .unwrap();
    }

    fn run(&self, req: &Request, res: &mut Response) -> Result<()> {
        let routes = self.methods.get(&req.method).ok_or(Error::new(
            ErrorKind::NotFound,
            concat_string!("method: ", req.method.as_ref()),
        ))?;

        let matched = routes.at(req.path.location).ok().ok_or(Error::new(
            ErrorKind::NotFound,
            concat_string!(
                "method: ",
                req.method.as_ref(),
                ", locoation: ",
                req.path.location
            ),
        ))?;

        (matched.value)(req, res, matched.params)
    }
}
