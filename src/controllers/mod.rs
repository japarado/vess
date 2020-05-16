use crate::errors::ServiceError;
use actix_web::error::BlockingError;
use actix_web::HttpResponse;
use serde::Deserialize;

pub mod auth_controller;
pub mod comment_controller;
pub mod post_controller;
pub mod post_tag_controller;
pub mod tag_controller;
pub mod user_controller;

#[derive(Deserialize)]
pub struct IdPath {
    id: i32,
}

pub type GenericRespnse = Result<HttpResponse, ServiceError>;

pub fn ok_response<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}
