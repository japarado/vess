use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::comment::Comment;
use crate::models::user::User;
use crate::models::{Multiple, Single};
use crate::repositories::user_repository;

pub fn show(conn: &Conn, id: &i32) -> Single<User> {
    Ok(user_repository::show(conn, id)?)
}
