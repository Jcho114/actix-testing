use crate::modules::auth::util;
use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpResponse,
};

pub async fn validate_middleware(
    request: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let access_cookie_option = request.cookie("access_token");
    if let None = access_cookie_option {
        return Ok(request
            .into_response(HttpResponse::Unauthorized().body("Access token is not provided")));
    }
    let access_cookie = access_cookie_option.unwrap().value().to_string();
    if let None = util::validate_access_token(access_cookie) {
        return Ok(request
            .into_response(HttpResponse::Unauthorized().body("Provided access token is invalid")));
    }
    next.call(request).await
}
