use diesel::prelude;

use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::comment::{Comment, NewComment};
use crate::models::{Multiple, Single};
use crate::repositories::comment_repository;
use crate::schema::{comments, users};

pub fn show(conn: &Conn, id: &i32) -> Single<Comment> {
    Ok(comment_repository::show(conn, id)?)
}
