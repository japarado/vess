use super::Conn;
use crate::models::post::{MultiplePosts, NewPost, Post, SinglePost};
use crate::schema::posts;
use diesel::prelude::*;

pub fn index(conn: &Conn) -> MultiplePosts {
    Ok(posts::table.order(posts::id.asc()).get_results(conn)?)
}
