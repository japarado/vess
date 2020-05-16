use super::ok_response;
use crate::database::StatePool;
use crate::errors::ServiceError;
use crate::models::post_tag::{MultiplePostTags, PostTag, SinglePostTag};
use crate::models::user::AuthUser;
use actix_web::error::BlockingError;
#[allow(unused_imports)]
use actix_web::{delete, get, patch, post, put, web, HttpResponse};

#[get("")]
pub async fn index(pool: StatePool) -> Result<HttpResponse, ServiceError> {
    web::block(move || -> MultiplePostTags { Ok(PostTag::index(pool)?) })
        .await
        .map(|post_tags| ok_response(post_tags))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[get("/{post_id}/{tag_id}")]
pub async fn show(
    pool: StatePool,
    path: web::Path<PostTag>,
    _auth_user: AuthUser,
) -> Result<HttpResponse, ServiceError> {
    web::block(move || -> SinglePostTag { Ok(PostTag::show(pool, &path.post_id, &path.tag_id)?) })
        .await
        .map(|post_tag| ok_response(post_tag))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}
