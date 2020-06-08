use crate::models::{post::Post, user::User};
use crate::schema::*;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, PartialEq, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub description: Option<String>,
    #[serde(skip)]
    pub user_id: i32,
}

// #[derive(Serialize, Debug)]
// pub struct FullTag {
//     pub name: String,
//     pub description: Option<String>,
//     user: User,
//     posts: Vec<Post>,
// }

impl From<Json<NewTag>> for NewTag {
    fn from(new_tag: Json<NewTag>) -> Self {
        Self {
            name: new_tag.name.to_owned(),
            description: new_tag.description.to_owned(),
            user_id: 0,
        }
    }
}
