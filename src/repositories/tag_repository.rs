use crate::database::Conn;
use crate::models::tag::{NewTag, Tag};
use crate::models::{Multiple, Single};
use crate::schema::tags;
use diesel::prelude::*;

pub fn index(conn: &Conn) -> Multiple<Tag> {
    Ok(tags::table.order(tags::id.asc()).get_results(conn)?)
}

pub fn store(conn: &Conn, new_tag: NewTag) -> Single<Tag> {
    Ok(diesel::insert_into(tags::table)
        .values(new_tag)
        .get_result(conn)?)
}
