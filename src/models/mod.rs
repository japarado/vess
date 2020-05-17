pub mod comment;
pub mod post;
pub mod post_tag;
pub mod tag;
pub mod user;

use crate::errors::ServiceError;

pub type Multiple<T> = Result<Vec<T>, ServiceError>;
pub type Single<T> = Result<T, ServiceError>;
