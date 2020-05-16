use super::{ok_response, GenericRespnse, IdPath};
use crate::database::StatePool;
use crate::errors::ServiceError;
use crate::models::comment::{MultipleComments, SingleComment};
use crate::models::{comment::Comment, comment::CommentRest, comment::NewComment, user::AuthUser};
use actix_web::error::BlockingError;
use actix_web::{delete, post, web};

#[post("")]
pub async fn store(
    pool: StatePool,
    payload: web::Json<CommentRest>,
    auth_user: AuthUser,
) -> GenericRespnse {
    let new_comment = NewComment {
        contents: payload.contents.clone(),
        post_id: payload.post_id,
        user_id: auth_user.id,
    };
    web::block(move || -> SingleComment { Ok(Comment::store(pool, new_comment)?) })
        .await
        .map(|comment| ok_response(comment))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[delete("/{id}")]
pub async fn destroy(
    pool: StatePool,
    path: web::Path<IdPath>,
    auth_user: AuthUser,
) -> GenericRespnse {
    web::block(move || -> SingleComment { Ok(Comment::destroy(pool, &path.id, &auth_user.id)?) })
        .await
        .map(|comment| ok_response(comment))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}
