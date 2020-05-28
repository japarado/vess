use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::comment::Comment;
use crate::models::post::{FullPost, NewPost, Post};
use crate::models::{Multiple, Single};
use crate::repositories::{comment_repository, post_repository, user_repository};

pub fn index(conn: &Conn) -> Multiple<Post> {
    Ok(post_repository::index(conn)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<FullPost> {
    let post = post_repository::show(conn, id)?;
    let user = user_repository::show(conn, &post.user_id)?;
    let comments = comment_repository::get_by_post_id(conn, &post.id)?;

    Ok(FullPost {
        id: post.id,
        title: post.title,
        body: post.body,
        user,
        comments,
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

pub fn update(conn: &Conn, id: &i32, user_id: &i32, new_post: NewPost) -> Single<Post> {
    let post = post_repository::show(conn, id)?;
    if &post.user_id == user_id {
        Ok(post_repository::update(conn, id, new_post)?)
    } else {
        Err(ServiceError::Unauthorized)
    }
}
