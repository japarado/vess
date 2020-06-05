use crate::database::Conn;
use crate::models::user::{NewUser, User};
use crate::models::{Multiple, Single};
use crate::repositories::user_repository;

pub fn index(conn: &Conn) -> Multiple<User> {
    Ok(user_repository::index(conn)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<User> {
    Ok(user_repository::show(conn, id)?)
}

pub fn update(conn: &Conn, id: &i32, new_user: NewUser) -> Single<User> {
    Ok(user_repository::update(conn, id, new_user)?)
}
