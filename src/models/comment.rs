use crate::models::{post::Post, user::User};
use crate::schema::comments;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Post, foreign_key = "post_id")]
pub struct Comment {
    pub id: i32,
    pub contents: String,
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "comments"]
pub struct NewComment {
    pub contents: String,
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Serialize, Debug)]
pub struct FullComment {
    pub id: i32,
    pub contents: String,
    pub user_id: i32,
    pub post_id: i32,
    pub user: User,
}
