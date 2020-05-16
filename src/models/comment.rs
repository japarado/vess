use crate::database::StatePool;
use crate::errors::ServiceError;
use crate::models::{post::Post, user::User};
use crate::schema::comments;
use actix_web::web::Json;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[belongs_to(Post)]
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

#[derive(Deserialize)]
pub struct CommentRest {
    pub contents: String,
    pub post_id: i32,
}

pub type SingleComment = Result<Comment, ServiceError>;
pub type MultipleComments = Result<Vec<Comment>, ServiceError>;

impl From<Json<CommentRest>> for CommentRest {
    fn from(comment_rest: Json<CommentRest>) -> Self {
        Self {
            contents: comment_rest.contents.clone(),
            post_id: comment_rest.post_id,
        }
    }
}

impl Comment {
    pub fn store(pool: StatePool, new_comment: NewComment) -> SingleComment {
        use crate::schema::comments::dsl::*;
        let conn = &pool.get().unwrap();
        Ok(diesel::insert_into(comments)
            .values(new_comment)
            .get_result(conn)?)
    }

    pub fn destroy(pool: StatePool, comment_pk: &i32, user_fk: &i32) -> SingleComment {
        let conn = &pool.get().unwrap();
        let target: Comment = comments::table
            .filter(comments::columns::id.eq(comment_pk))
            .get_result(conn)?;
        if &target.user_id == user_fk {
            Err(ServiceError::Unauthorized)
        } else {
            Ok(diesel::delete(&target).get_result(conn)?)
        }
    }

    pub fn find_by_post_id(pool: StatePool, post_fk: &i32) -> MultipleComments {
        let conn = &pool.get().unwrap();
        Ok(comments::table
            .filter(comments::post_id.eq(post_fk))
            .get_results(conn)?)
    }
}
