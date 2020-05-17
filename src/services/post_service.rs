use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::post::{NewPost, Post, PostWithComments};
use crate::models::{Multiple, Single};
use crate::repositories::comment_repository;
use crate::repositories::post_repository;

pub fn index(conn: &Conn) -> Multiple<Post> {
    Ok(post_repository::index(conn)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<PostWithComments> {
    let post = post_repository::show(conn, id)?;
    let comments = comment_repository::get_by_post_id(conn, id)?;
    Ok(PostWithComments {
        id: post.id,
        title: post.title,
        body: post.body,
        user_id: post.user_id,
        comments: Some(comments),
    })
}

pub fn store(conn: &Conn, new_post: NewPost) -> Single<Post> {
    Ok(post_repository::store(conn, new_post)?)
}

pub fn destroy(conn: &Conn, post_id: &i32, user_id: &i32) -> Single<Post> {
    let post = post_repository::show(conn, post_id)?;
    if &post.user_id != user_id {
        Err(ServiceError::Unauthorized)
    } else {
        Ok(post_repository::destroy(conn, post_id)?)
    }
}
