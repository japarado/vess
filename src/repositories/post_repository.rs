use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::post::{NewPost, Post};
use crate::models::{Multiple, Single};
use crate::schema::posts;
use diesel::prelude::*;

pub fn index(conn: &Conn) -> Multiple<Post> {
    Ok(posts::table.order(posts::id.asc()).load(conn)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<Post> {
    Ok(posts::table.find(id).first(conn)?)
}

pub fn store(conn: &Conn, new_post: NewPost) -> Single<Post> {
    Ok(diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(conn)?)
}

pub fn destroy(conn: &Conn, id: &i32) -> Single<Post> {
    Ok(diesel::delete(posts::table.find(id)).get_result(conn)?)
}
