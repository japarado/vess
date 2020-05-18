use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::comment::{Comment, NewComment};
use crate::models::{Multiple, Single};
use crate::schema::comments;
use diesel::prelude::*;

pub fn get_by_post_id(conn: &Conn, pid: &i32) -> Multiple<Comment> {
    Ok(comments::table
        .filter(comments::post_id.eq(pid))
        .get_results(conn)?)
}
