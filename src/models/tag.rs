use crate::schema::*;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, PartialEq, Debug)]
pub struct Tag {
    id: i32,
    name: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "tags"]
pub struct NewTag {
    name: String,
    description: Option<String>,
}

impl From<Json<NewTag>> for NewTag {
    fn from(new_tag: Json<NewTag>) -> Self {
        Self {
            name: new_tag.name.to_owned(),
            description: new_tag.description.to_owned(),
        }
    }
}
