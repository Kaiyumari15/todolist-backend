use rocket::Responder;

pub mod auth;
pub mod todotask;
pub mod user;

#[derive(Debug, Responder)]
/// Response types for the API
/// 
/// # Variants
/// * `Ok` - Indicates a successful request with a 200 status code and JSON content type.
/// * `Created` - Indicates a successful request that resulted in a resource being created, with a 201 status code and JSON content type.
/// * `BadRequest` - Indicates a client error with a 400 status code and text content type, along with an error message.
/// * `Unauthorized` - Indicates an authentication error with a 401 status code and text content type, along with an error message.
/// * `Forbidden` - Indicates a permission error with a 403 status code and text content type, along with an error message.
/// * `NotFound` - Indicates a resource not found error with a 404 status code and text content type, along with an error message.
/// * `InternalServerError` - Indicates a server error with a 500 status code and text content type, along with an error message.
pub enum Response<T> {
    #[response(status = 200, content_type = "json")]
    Ok(T),
    #[response(status = 201, content_type = "json")]
    Created(T),
    #[response(status = 400, content_type = "text")]
    BadRequest(String),
    #[response(status = 401, content_type = "text")]
    Unauthorized(String),
    #[response(status = 403, content_type = "text")]
    Forbidden(String),
    #[response(status = 404, content_type = "text")]
    NotFound(String),
    #[response(status = 500, content_type = "text")]
    InternalServerError(String),
}