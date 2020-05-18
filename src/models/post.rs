use crate::models::comment::Comment;
use crate::models::user::User;
use crate::schema::posts;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    title: String,
    body: Option<String>,
    #[serde(skip)]
    pub user_id: i32,
}

#[derive(Serialize, Debug)]
pub struct FullPost {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub comments: Vec<Comment>,
}

impl From<Json<NewPost>> for NewPost {
    fn from(new_post: Json<NewPost>) -> Self {
        Self {
            title: new_post.title.to_owned(),
            body: new_post.body.to_owned(),
            user_id: -1,
        }
    }
}
