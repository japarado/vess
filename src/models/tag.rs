use crate::database::StatePool;
use crate::errors::ServiceError;
use crate::models::post_tag::PostTag;
use crate::schema::*;
use actix_web::web::Json;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub type SingleTag = Result<Tag, ServiceError>;
pub type MultipleTags = Result<Vec<Tag>, ServiceError>;

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

impl Tag {
    pub fn index(pool: StatePool) -> MultipleTags {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        Ok(tags.order(id.asc()).load(conn)?)
    }

    pub fn store(pool: StatePool, new_tag: NewTag) -> SingleTag {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        Ok(diesel::insert_into(tags).values(new_tag).get_result(conn)?)
    }

    pub fn destroy(pool: StatePool, tag_pk: &i32) -> SingleTag {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        Ok(diesel::delete(tags.find(tag_pk)).get_result(conn)?)
    }
}
