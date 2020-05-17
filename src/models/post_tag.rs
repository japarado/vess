use crate::models::{post::Post, tag::Tag};
use crate::schema::*;
use serde::{Deserialize, Serialize};

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
