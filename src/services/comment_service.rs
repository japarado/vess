use diesel::prelude;

use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::comment::{Comment, FullComment, NewComment};
use crate::models::{Multiple, Single};
use crate::repositories::comment_repository;
use crate::schema::{comments, users};

pub fn show(conn: &Conn, id: &i32) -> Single<FullComment> {
    Ok(comment_repository::show(conn, id)?)
}

pub fn destroy(conn: &Conn, id: &i32, user_id: &i32) -> Single<Comment> {
    Ok(comment_repository::destroy(conn, id, user_id)?)
}

pub fn store(conn: &Conn, new_comment: NewComment) -> Single<Comment> {
    Ok(comment_repository::store(conn, new_comment)?)
}
