use super::{ok_closure, AppData, GenericResponse, IdPath};
use crate::errors::ServiceError;
use crate::models::user::{AuthUser, NewUser, ResetPasswordRequest, User};
use crate::models::{Multiple, Single};
use crate::services::user_service;
use actix_web::error::BlockingError;
use actix_web::{delete, get, patch, post, web, HttpResponse};

#[get("")]
pub async fn index(data: AppData) -> GenericResponse {
    web::block(move || -> Multiple<User> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(user_service::index(conn)?)
    })
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

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

#[patch("/{id}")]
pub async fn update(
    data: AppData,
    path: web::Path<IdPath>,
    payload: web::Json<NewUser>,
) -> GenericResponse {
    web::block(move || -> Single<User> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(user_service::update(conn, &path.id, payload.into())?)
    })
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[patch("/reset-password")]
pub async fn reset_password(
    data: AppData,
    payload: web::Json<ResetPasswordRequest>,
    auth_user: AuthUser,
) -> GenericResponse {
    web::block(move || -> Single<User> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(user_service::reset_password(
            conn,
            &auth_user.id,
            payload.into(),
        )?)
    })
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}
