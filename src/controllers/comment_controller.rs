use actix_web::error::BlockingError;
use actix_web::{get, web};

use crate::errors::ServiceError;
use crate::models::comment::Comment;
use crate::models::{Multiple, Single};
use crate::services::comment_service;

use super::{ok_closure, AppData, GenericResponse, IdPath};

#[get("/{id}")]
pub async fn show(data: AppData, path: web::Path<IdPath>) -> GenericResponse {
    web::block(move || -> Single<Comment> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(comment_service::show(conn, &path.id)?)
    })
    .await
    .map(|comment| ok_closure(comment))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}
