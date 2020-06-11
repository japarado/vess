use crate::controllers::IdPath;
use crate::controllers::{ok_closure, AppData, GenericResponse};
use crate::errors::ServiceError;
use crate::models::tag::{NewTag, Tag};
use crate::models::user::AuthUser;
use crate::models::{Multiple, Single};
use crate::repositories::tag_repository;
use crate::services::tag_service;
use actix_web::error::BlockingError;
use actix_web::{get, post, web};

#[get("")]
pub async fn index(data: AppData) -> GenericResponse {
    web::block(move || -> Multiple<Tag> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(tag_repository::index(conn)?)
    })
    .await
    .map(|tags| ok_closure(tags))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[post("")]
pub async fn store(
    data: AppData,
    payload: web::Json<NewTag>,
    auth_user: AuthUser,
) -> GenericResponse {
    web::block(move || -> Single<Tag> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        let mut new_tag: NewTag = payload.into();
        new_tag.user_id = auth_user.id;
        Ok(tag_repository::store(conn, new_tag)?)
    })
    .await
    .map(|tags| ok_closure(tags))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[get("/{id}")]
pub async fn show(data: AppData, path: web::Path<IdPath>) -> GenericResponse {
    web::block(move || -> Single<Tag> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(tag_service::show(conn, &path.id)?)
    })
    .await
    .map(|tag| ok_closure(tag))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}
