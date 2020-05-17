use actix_web::web::Data;
use std::sync::Mutex;

use crate::errors::ServiceError;
use crate::ApplicationData;
use actix_web::error::{BlockingError, Error};
use actix_web::HttpResponse;
use serde::Deserialize;

pub mod auth_controller;
pub mod comment_controller;
pub mod post_controller;
pub mod post_tag_controller;
pub mod tag_controller;
pub mod user_controller;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct IdPath {
    id: i32,
}

pub type AppData = Data<Mutex<ApplicationData>>;
pub type GenericResponse = Result<HttpResponse, ServiceError>;

pub fn ok_closure<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}

// pub fn error_closure(err: BlockingError<Error>) -> HttpResponse {
//     match err {
//         BlockingError::Error(service_error) => service_error,
//         BlockingError::Canceled => ServiceError::InternalServerError,
//     }
// }
