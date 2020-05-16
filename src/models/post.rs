use crate::database::StatePool;
use crate::errors::ServiceError;
use crate::models::comment::Comment;
use crate::models::user::User;
use crate::schema::posts;
use actix_web::web::Json;
use diesel::prelude::*;
use diesel::result::Error;
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
    pub user_id: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct PostWithComments {
    pub title: String,
    pub body: Option<String>,
    pub user_id: i32,
    pub comments: Option<Vec<Comment>>,
}

impl From<Json<NewPost>> for NewPost {
    fn from(new_post: Json<NewPost>) -> Self {
        Self {
            title: new_post.title.to_owned(),
            body: new_post.body.to_owned(),
            user_id: Some(-1),
        }
    }
}

pub type MultiplePosts = Result<Vec<Post>, ServiceError>;
pub type SinglePost = Result<Post, ServiceError>;

impl Post {
    pub fn index(pool: StatePool) -> Result<Vec<Self>, ServiceError> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        Ok(posts.order(id.asc()).load::<Self>(conn)?)
    }

    pub fn mine(pool: StatePool, user_fk: &i32) -> Result<Vec<Self>, Error> {
        let conn = &pool.get().unwrap();
        use crate::schema::posts::dsl::*;
        Ok(posts.filter(user_id.eq(user_fk)).get_results(conn)?)
    }

    pub fn show(pool: StatePool, post_pk: &i32) -> Result<Self, ServiceError> {
        let conn = &pool.get().unwrap();
        Ok(posts::table.find(post_pk).get_result(conn)?)
    }

    pub fn store(pool: StatePool, new_post: NewPost) -> Result<Self, ServiceError> {
        let conn = &pool.get().unwrap();
        use crate::schema::posts::dsl::*;
        Ok(diesel::insert_into(posts)
            .values(new_post)
            .get_result(conn)?)
    }

    pub fn update(
        pool: StatePool,
        post_pk: &i32,
        user_fk: &i32,
        updated_post: NewPost,
    ) -> Result<Self, ServiceError> {
        let conn = &pool.get().unwrap();
        let target_post = posts::table.find(post_pk).first::<Post>(conn)?;
        if target_post.user_id == user_fk.clone() {
            Ok(diesel::update(&target_post)
                .set(updated_post)
                .get_result(conn)?)
        } else {
            Err(ServiceError::Unauthorized)
        }
    }

    pub fn destroy(pool: StatePool, post_pk: &i32, user_fk: &i32) -> Result<Self, ServiceError> {
        let conn = &pool.get().unwrap();
        let target_post = posts::table.find(post_pk).first::<Self>(conn)?;
        if &target_post.user_id == user_fk {
            Ok(diesel::delete(&target_post).get_result(conn)?)
        } else {
            Err(ServiceError::Unauthorized)
        }
    }
}
