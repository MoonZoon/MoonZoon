use actix_web::dev::{Service, Transform, forward_ready};
use std::{future::{ready, Ready}};

// ------ Redirect ------

#[derive(Copy, Clone)]
pub struct Redirect;

impl Redirect {
    pub fn new() -> Self {
        Self
    }

    pub fn http_to_https(self, http_to_https: bool) -> Self {
        self
    } 

    pub fn port(self, from_port: u16, to_port: u16) -> Self {
        self
    } 
}


impl<S: Service<Req>, Req> Transform<S, Req> for Redirect {
    type Response = S::Response;
    type Error = S::Error;
    type InitError = ();
    type Transform = RedirectMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RedirectMiddleware {
            service,
            redirect: *self,
        }))
    }
}

// ------ RedirectMiddleware ------

pub struct RedirectMiddleware<S> {
    service: S,
    redirect: Redirect
}

impl<S: Service<Req>, Req> Service<Req> for RedirectMiddleware<S> {
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: Req) -> Self::Future {
        self.service.call(req)
    }
}
