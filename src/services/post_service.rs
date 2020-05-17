use crate::database::StatePool;
use crate::models::post::{MultiplePosts, NewPost, Post, PostWithComments, SinglePost};
use crate::repositories::post_repository;

trait IPostService {
    fn index(pool: StatePool) -> MultiplePosts;
    fn show(pool: StatePool, post_pk: &i32) -> PostWithComments;
    fn store(pool: StatePool, new_post: NewPost) -> SinglePost;
    fn destroy(pool: StatePool, post_pk: &i32) -> SinglePost;
    fn update(pool: StatePool, post_pk: &i32, updated_post: NewPost);
}

pub fn index(pool: StatePool) -> MultiplePosts {
    Ok(post_repository::index(&pool.get().unwrap())?)
}
