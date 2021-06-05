use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::LOCATION;
use actix_web::http::uri::{Authority, InvalidUriParts, Scheme, Uri};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use std::convert::TryFrom;
use bool_ext::BoolExt;

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
    redirect: Redirect,
}

impl<S> RedirectMiddleware<S> {
    fn uri(req: &ServiceRequest) -> Result<Uri, InvalidUriParts> {
        let connection_info = req.connection_info();

        // Note: "http/1 does not send host in uri" (https://github.com/actix/actix-web/issues/1111)
        let mut uri_parts = req.uri().clone().into_parts();
        uri_parts.scheme = Scheme::try_from(connection_info.scheme()).ok();
        uri_parts.authority = Authority::try_from(connection_info.host()).ok();

        Uri::from_parts(uri_parts)
    }

    fn should_redirect(&self, uri: &Uri) -> Option<()> {
        let from_port = self.redirect.from_port;

        match (uri.scheme()?, uri.authority()?.port_u16()) {
            (_, Some(port)) => from_port == port,
            (scheme, None) if scheme == &Scheme::HTTP => from_port == 80,
            (scheme, None) if scheme == &Scheme::HTTPS => from_port == 443,
            _ => None?,
        }.to_option()
    }

    fn redirect_uri(&self, uri: Uri) -> Option<Uri> {
        let mut uri_parts = uri.into_parts();

        if self.redirect.http_to_https && uri_parts.scheme.as_ref()? == &Scheme::HTTP {
            uri_parts.scheme = Some(Scheme::HTTPS);
        }
        uri_parts.authority = Authority::try_from(
            format!("{}:{}", uri_parts.authority?.host(), self.redirect.to_port).as_str(),
        )
        .ok();

        Uri::from_parts(uri_parts).ok()
    }

    fn redirect<B>(
        &self,
        req: ServiceRequest,
        uri: &Uri,
    ) -> Ready<Result<ServiceResponse<B>, Error>> {
        let http_response = HttpResponse::MovedPermanently()
            .insert_header((LOCATION, uri.to_string()))
            .finish()
            .into_body();

        ok(req.into_response(http_response))
    }
}

impl<S, B> Service<ServiceRequest> for RedirectMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Ok(uri) = Self::uri(&req) {
            if self.should_redirect(&uri).is_some() {
                let redirect_uri = self.redirect_uri(uri).unwrap();
                return Either::Right(self.redirect(req, &redirect_uri));
            }
        }
        Either::Left(self.service.call(req))
    }
}
