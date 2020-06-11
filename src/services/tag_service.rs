use crate::database::Conn;
use crate::models::tag::{FullTag, NewTag, Tag};
use crate::models::{Multiple, Single};
use crate::repositories::tag_repository;

pub fn index(conn: &Conn) -> Multiple<Tag> {
    Ok(tag_repository::index(conn)?)
}

pub fn store(conn: &Conn, new_tag: NewTag) -> Single<Tag> {
    Ok(tag_repository::store(conn, new_tag)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<Tag> {
    Ok(tag_repository::show(conn, id)?)
}
