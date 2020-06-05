use super::{ok_closure, AppData, GenericResponse, IdPath};
use crate::errors::ServiceError;
use crate::models::post::{FullPost, NewPost, Post};
use crate::models::user::AuthUser;
use crate::models::{Multiple, Single};
use crate::services::post_service;
use actix_web::error::BlockingError;
use actix_web::{delete, get, patch, post, web, HttpResponse};

#[get("")]
pub async fn index(data: AppData) -> GenericResponse {
    web::block(move || -> Multiple<Post> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(post_service::index(conn)?)
    })
    .await
    .map(|posts| HttpResponse::Ok().json(posts))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[get("/{id}")]
pub async fn show(data: AppData, path: web::Path<IdPath>) -> GenericResponse {
    web::block(move || -> Single<FullPost> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(post_service::show(conn, &path.id)?)
    })
    .await
    .map(|post| ok_closure(post))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[post("")]
pub async fn store(
    data: AppData,
    payload: web::Json<NewPost>,
    auth_user: AuthUser,
) -> GenericResponse {
    web::block(move || -> Single<Post> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        let mut new_post: NewPost = payload.into();
        new_post.user_id = auth_user.id;
        Ok(post_service::store(conn, new_post)?)
    })
    .await
    .map(|post| ok_closure(post))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[delete("/{id}")]
pub async fn destroy(
    data: AppData,
    path: web::Path<IdPath>,
    auth_user: AuthUser,
) -> GenericResponse {
    web::block(move || -> Single<Post> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(post_service::destroy(conn, &path.id, &auth_user.id)?)
    })
    .await
    .map(|post| ok_closure(post))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[patch("/{id}")]
pub async fn update(
    data: AppData,
    path: web::Path<IdPath>,
    payload: web::Json<NewPost>,
    auth_user: AuthUser,
) -> GenericResponse {
    web::block(move || -> Single<Post> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        let mut new_post: NewPost = payload.into();
        new_post.user_id = auth_user.id;
        Ok(post_service::update(
            conn,
            &path.id,
            &auth_user.id,
            new_post,
        )?)
    })
    .await
    .map(|post| ok_closure(post))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

// #[get("/mine")]
// pub async fn mine(data: AppData, path: web::Path<Data>, auth_user: AuthUser) -> GenericResponse {
//     web::block(move || -> Multiple<Post> {
//         let data = data.lock().unwrap();
//         let conn = &data.conn_pool.get().unwrap();
//     })
// }
