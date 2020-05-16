use crate::database::StatePool;
use crate::errors::ServiceError;
use crate::models::{post::Post, tag::Tag};
use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub type SinglePostTag = Result<PostTag, ServiceError>;
pub type MultiplePostTags = Result<Vec<PostTag>, ServiceError>;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[primary_key(post_id, tag_id)]
#[belongs_to(Post, foreign_key = "post_id")]
#[belongs_to(Tag, foreign_key = "tag_id")]
pub struct PostTag {
    pub post_id: i32,
    pub tag_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "post_tags"]
pub struct NewPostTag {
    pub post_id: i32,
    pub tag_id: i32,
}

impl PostTag {
    pub fn index(pool: StatePool) -> Result<Vec<Self>, ServiceError> {
        use crate::schema::post_tags::dsl::*;
        let conn = &pool.get().unwrap();
        Ok(post_tags.order(post_id.asc()).get_results(conn)?)
    }

    pub fn show(pool: StatePool, post_pk: &i32, tag_pk: &i32) -> Result<Self, ServiceError> {
        use crate::schema::post_tags::dsl::*;
        let conn = &pool.get().unwrap();
        Ok(post_tags.find((post_pk, tag_pk)).get_result(conn)?)
    }

    // pub fn post_tags(pool: StatePool, post_pk: &i32) -> Result<Vec<Self>, ServiceError> {
    //     use crate::schema::post_tags::dsl::*;
    //     use crate::schema::tags::dsl::*;
    //     let conn = &pool.get().unwrap();
    //     let post = Post::show(pool, post_pk)?;
    //     Ok(PostTag::belonging_to(&post).inner_join(tags).get_results(conn)?)
    // }
    // pub fn post_tags(pool: StatePool, post_pk: &i32) -> Result<Vec<Tag>, ServiceError> {
    //     use crate::schema::tags::dsl::*;
    //     let conn = &pool.get().unwrap();
    //     let post_tags: Vec<PostTag> = Self::by_post(pool, post_pk)?;
    //     let mut tag_ids: Vec<i32> = Vec::new();
    //     for post_tag in post_tags.iter() {
    //         tag_ids.push(post_tag.tag_id);
    //     }
    //     Ok(tags.filter(id.eq_any(tag_ids)).get_results::<Tag>(conn)?)
    // }

    // fn by_post(pool: StatePool, post_pk: &i32) -> Result<Vec<Self>, ServiceError> {
    //     use crate::schema::post_tags::dsl::*;
    //     let conn = &pool.get().unwrap();
    //     Ok(post_tags.filter(post_id.eq(post_pk)).get_results(conn)?)
    // }

    // fn by_tag(pool: StatePool, tag_pk: &i32) -> Result<Vec<Self>, ServiceError> {
    //     use crate::schema::post_tags::dsl::*;
    //     let conn = &pool.get().unwrap();
    //     Ok(post_tags.filter(post_id.eq(tag_pk)).get_results(conn)?)
    // }
}
