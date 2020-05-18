use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::user::User;
use crate::models::{Multiple, Single};
use crate::schema::users;
use diesel::prelude::*;

pub fn show(conn: &Conn, id: &i32) -> Single<User> {
    Ok(users::table.find(id).first(conn)?)
}
