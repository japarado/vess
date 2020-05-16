use super::ok_response;
use super::IdPath;
use crate::database::StatePool;
use crate::errors::ServiceError;
use crate::models::tag::{MultipleTags, NewTag, SingleTag, Tag};
use crate::models::user::AuthUser;
use actix_web::error::BlockingError;

#[allow(unused_imports)]
use actix_web::{delete, get, patch, post, web, HttpResponse};

#[get("")]
pub async fn index(pool: StatePool) -> Result<HttpResponse, ServiceError> {
    web::block(move || -> MultipleTags { Ok(Tag::index(pool)?) })
        .await
        .map(|tags| ok_response(tags))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[post("")]
pub async fn store(
    pool: StatePool,
    payload: web::Json<NewTag>,
    _auth_user: AuthUser,
) -> Result<HttpResponse, ServiceError> {
    web::block(move || -> SingleTag { Ok(Tag::store(pool, payload.into())?) })
        .await
        .map(|tag| ok_response(tag))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[delete("/{id}")]
pub async fn destroy(
    pool: StatePool,
    path: web::Path<IdPath>,
    _auth_user: AuthUser,
) -> Result<HttpResponse, ServiceError> {
    web::block(move || -> SingleTag { Ok(Tag::destroy(pool, &path.id)?) })
        .await
        .map(|tag| ok_response(tag))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}
