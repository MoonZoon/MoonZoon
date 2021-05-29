use actix_web::dev::{Service, Transform, forward_ready, ServiceRequest, ServiceResponse};
use actix_web::{HttpResponse, Error};
use futures::future::{Either, ok, Ready};

// ------ Redirect ------

#[derive(Copy, Clone)]
pub struct Redirect {
    http_to_https: bool,
    from_port: u16,
    to_port: u16,
}

impl Default for Redirect {
    fn default() -> Self {
        Self {
            http_to_https: true,
            from_port: 80,
            to_port: 443,
        }
    }
}

impl Redirect {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn http_to_https(mut self, http_to_https: bool) -> Self {
        self.http_to_https = http_to_https;
        self
    } 

    pub fn port(mut self, from_port: u16, to_port: u16) -> Self {
        self.from_port = from_port;
        self.to_port = to_port;
        self
    } 
}


impl<S, B> Transform<S, ServiceRequest> for Redirect
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Response = S::Response;
    type Error = S::Error;
    type InitError = ();
    type Transform = RedirectMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectMiddleware {
            service,
            redirect: *self,
        })
    }
}

// ------ RedirectMiddleware ------

pub struct RedirectMiddleware<S> {
    service: S,
    redirect: Redirect
}

impl<S> RedirectMiddleware<S> {
    fn should_redirect(&self, req: &ServiceRequest) -> bool {
        println!("Scheme: {}", req.connection_info().scheme());
        println!("Scheme 2: {:#?}", req.uri().scheme());
        println!("Port: {:#?}", req.uri().port_u16());
        true
    }

    fn redirect<B>(&self, req: ServiceRequest) -> Ready<Result<ServiceResponse<B>, Error>> {
        ok(req.into_response(HttpResponse::Ok().finish().into_body()))

        // ok(req.into_response(
        //     HttpResponse::MovedPermanently()
        //         .header(http::header::LOCATION, url)
        //         .finish()
        //         .into_body(),
        // ))
    }
}

impl<S, B> Service<ServiceRequest> for RedirectMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Either<Ready<Result<Self::Response, Self::Error>>, S::Future>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.should_redirect(&req) {
            return Either::Left(self.redirect(req))
        }
        Either::Right(self.service.call(req))
    }
}
