use super::{ok_closure, AppData, GenericResponse, IdPath};
use crate::errors::ServiceError;
use crate::models::user::User;
use crate::models::{Multiple, Single};
use crate::services::user_service;
use actix_web::error::BlockingError;
use actix_web::{delete, get, patch, post, web, HttpResponse};

#[get("/{id}")]
pub async fn show(data: AppData, path: web::Path<IdPath>) -> GenericResponse {
    web::block(move || -> Single<User> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(user_service::show(conn, &path.id)?)
    })
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}
