use crate::models::{post::Post, user::User};
use crate::schema::comments;
use actix_web::web::Json;
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
    #[serde(skip_deserializing)]
    pub user_id: i32,
    pub post_id: i32,
}

#[derive(Serialize, Queryable, Debug)]
pub struct FullComment {
    pub id: i32,
    pub contents: String,
    pub user_id: i32,
    pub post_id: i32,
    pub user: User,
}

// impl From<Json<NewComment>> for NewComment {
//     fn from(new_post: Json<NewComment>) -> Self {
//         Self {
//             contents: new_post.contents.to_owned(),
//             user_id: new_post.user_id,
//             post_id: new_post.post_id,
//         }
//     }
// }
