use crate::database::Conn;
use crate::models::post::{NewPost, Post};
use crate::models::tag::Tag;
use crate::models::{Multiple, Single};
use crate::schema::posts;
use diesel::prelude::*;

pub fn index(conn: &Conn) -> Multiple<Post> {
    Ok(posts::table.order(posts::id.asc()).load(conn)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<Post> {
    // let queried_posts = posts::table.filter(posts::id.contains([1,2])).get_results(conn)?;
    // println!("{:#?}",queried_posts);
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

pub fn update(conn: &Conn, id: &i32, new_post: NewPost) -> Single<Post> {
    Ok(diesel::update(posts::table.find(id))
        .set(new_post)
        .get_result(conn)?)
}
