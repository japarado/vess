use crate::database::StatePool;
use crate::errors::ServiceError;
use crate::models::comment::Comment;
use crate::models::post::{MultiplePosts, NewPost, Post, PostWithComments, SinglePost};
use crate::models::post_tag::{MultiplePostTags, PostTag, SinglePostTag};
use crate::models::user::AuthUser;
use crate::models::user::User;
use diesel::prelude::*;

use actix_web::error::BlockingError;

use super::ok_response;
use super::IdPath;

use diesel::prelude::*;

#[allow(unused_imports)]
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("")]
pub async fn index(pool: StatePool) -> Result<HttpResponse, ServiceError> {
    web::block(move || -> MultiplePosts { Ok(Post::index(pool)?) })
        .await
        .map(|posts| ok_response(posts))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[get("/mine")]
pub async fn mine(pool: StatePool, auth_user: AuthUser) -> Result<HttpResponse, ServiceError> {
    web::block(move || -> MultiplePosts { Ok(Post::mine(pool, &auth_user.id)?) })
        .await
        .map(|posts| ok_response(posts))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[get("/{id}")]
pub async fn show(pool: StatePool, path: web::Path<IdPath>) -> impl Responder {
    use crate::schema::{comments, posts};
    let conn = &pool.get().unwrap();

    match posts::table.find(&path.id).first::<Post>(conn) {
        Ok(target_post) => match Comment::belonging_to(&target_post).get_results::<Comment>(conn) {
            Ok(post_comments) => {
                let post_with_comments = PostWithComments {
                    title: target_post.title,
                    body: target_post.body,
                    user_id: target_post.user_id,
                    comments: Some(post_comments),
                };
                HttpResponse::Ok().json(post_with_comments)
            }
            Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().json(err.to_string()),
    }
}
// #[get("/{id}")]
// pub async fn show(pool: StatePool, path: web::Path<IdPath>) -> Result<HttpResponse, ServiceError> {
//     web::block(move || -> SinglePost { Ok(Post::show(pool, &path.id)?) })
//         .await
//         .map(|post| ok_response(post))
//         .map_err(|err| match err {
//             BlockingError::Error(service_error) => service_error,
//             BlockingError::Canceled => ServiceError::InternalServerError,
//         })
// }

#[post("")]
pub async fn store(
    pool: StatePool,
    auth_user: AuthUser,
    payload: web::Json<NewPost>,
) -> Result<HttpResponse, ServiceError> {
    let mut new_post: NewPost = payload.into();
    new_post.user_id = Some(auth_user.id);
    web::block(move || -> SinglePost { Ok(Post::store(pool, new_post)?) })
        .await
        .map(|post| ok_response(post))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[patch("/{id}")]
pub async fn update(
    pool: StatePool,
    post_pk: web::Path<IdPath>,
    post: web::Json<NewPost>,
    auth_user: AuthUser,
) -> Result<HttpResponse, ServiceError> {
    let mut updated_post: NewPost = post.into();
    updated_post.user_id = Some(auth_user.id);
    web::block(move || -> SinglePost {
        Ok(Post::update(
            pool,
            &post_pk.id,
            &auth_user.id,
            updated_post,
        )?)
    })
    .await
    .map(|post| ok_response(post))
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
) -> Result<HttpResponse, ServiceError> {
    web::block(move || -> SinglePost { Ok(Post::destroy(pool, &path.id, &auth_user.id)?) })
        .await
        .map(|post| ok_response(post))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[get("/sample")]
pub async fn sample(pool: StatePool) -> Result<HttpResponse, ServiceError> {
    use crate::schema::posts;
    use crate::schema::users;
    let conn = &pool.get().unwrap();
    let join_results: Vec<(User, Post)> =
        users::table.inner_join(posts::table).get_results(conn)?;
    Ok(HttpResponse::Ok().json(join_results))
}
// #[get("/sample")]
// pub async fn sample(pool: StatePool) -> Result<HttpResponse, ServiceError> {
//     use crate::schema::posts;
//     use crate::schema::users;
//     let conn = &pool.get().unwrap();

//     let users: Vec<User> = users::table.order(users::id.asc()).get_results(conn)?;

//     let posts = Post::belonging_to(&users)
//         .load::<Post>(conn)?
//         .grouped_by(&users);
//     let data = users.into_iter().zip(posts).collect::<Vec<_>>();
//     Ok(HttpResponse::Ok().json(data))
// }
// #[get("/sample")]
// pub async fn sample(pool: StatePool) -> Result<HttpResponse, ServiceError> {
//     use crate::schema::posts;
//     use crate::schema::users;
//     let conn = &pool.get().unwrap();

//     let user: User = users::table.find(1).first(conn)?;
//     let join_results: Vec<(String, String)> = users::table.inner_join(posts::table)
//         .select((users::email, posts::title))
//         .get_results(conn)?;
//     Ok(HttpResponse::Ok().json(join_results))
// }
