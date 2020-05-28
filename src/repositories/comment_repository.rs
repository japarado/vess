use crate::database::Conn;
use crate::models::comment::{Comment, FullComment, NewComment};
use crate::models::{Multiple, Single};
use crate::schema::comments;
use diesel::prelude::*;

use crate::models::post::Post;

use crate::models::user::User;
use crate::schema::users;

pub fn index(conn: &Conn) -> Multiple<Comment> {
    Ok(comments::table
        .order(comments::id.asc())
        .get_results(conn)?)
}

pub fn destroy(conn: &Conn, id: &i32, user_id: &i32) -> Single<Comment> {
    Ok(diesel::delete(
        comments::table
            .filter(comments::id.eq(id))
            .filter(comments::user_id.eq(user_id)),
    )
    .get_result(conn)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<FullComment> {
    let joined: (Comment, User) = comments::table
        .find(id)
        .inner_join(users::table)
        .first(conn)?;
    let comment = joined.0;
    let user = joined.1;
    Ok(FullComment {
        id: comment.id,
        contents: comment.contents,
        user_id: comment.user_id,
        post_id: comment.post_id,
        user: user,
    })
}

pub fn update(conn: &Conn, id: &i32, new_comment: NewComment) -> Single<Comment> {
    Ok(diesel::update(comments::table.find(id))
        .set(new_comment)
        .get_result(conn)?)
}

pub fn get_by_post_id(conn: &Conn, post_id: &i32) -> Multiple<FullComment> {
    let mut full_comments = Vec::new();
    let comments_user_pairs: Vec<(Comment, User)> = comments::table
        .inner_join(users::table)
        .filter(comments::post_id.eq(post_id))
        .get_results(conn)?;

    for comment_user in comments_user_pairs.into_iter() {
        let comment = comment_user.0;
        let user = comment_user.1;
        full_comments.push(FullComment {
            id: comment.id,
            contents: comment.contents,
            user_id: comment.user_id,
            post_id: comment.post_id,
            user,
        });
    }

    Ok(full_comments)
}
