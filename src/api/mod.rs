use rocket::Responder;

pub mod auth;
pub mod todotask;
pub mod user;

#[derive(Debug, Responder)]
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