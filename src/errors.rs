use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use serde_json::error::Error as SerdeError;
use std::convert::From;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmr = "Internal Sever Error")]
    InternalServerError,

    #[display(fmt = "Bad Requst: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Not Found")]
    NotFound(String),

    #[display(fmt = "Conflict")]
    Conflict(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            ServiceError::Conflict(ref message) => HttpResponse::Conflict().json(message),
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> Self {
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            DBError::NotFound => ServiceError::NotFound("Not Found. Custom".to_string()),
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<SerdeError> for ServiceError {
    fn from(error: SerdeError) -> Self {
        match error {
            _ => ServiceError::InternalServerError,
        }
    }
}
