use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
	NotFound,
    Unauthorized
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().json("InternalServerError"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
			ServiceError::NotFound => HttpResponse::NotFound().json("Not Found"),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}
