use actix_web::web::Data;
use std::sync::Mutex;

#[allow(unused_imports)]
use crate::errors::ServiceError;
use crate::ApplicationData;
#[allow(unused_imports)]
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
pub type GenericRespnse = Result<HttpResponse, ServiceError>;

pub fn ok_response<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}
