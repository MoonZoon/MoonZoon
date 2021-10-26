use actix_web::dev::ServiceResponse;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::Result;

pub fn internal_server_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    eprintln!("INTERNAL_SERVER_ERROR: {:?}", res.request().uri());
    Ok(ErrorHandlerResponse::Response(res))
}

pub fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    eprintln!("NOT_FOUND: {:?}", res.request().uri());
    Ok(ErrorHandlerResponse::Response(res))
}
