use crate::database::Conn;
use crate::models::user::{NewUser, User};
use crate::models::{Multiple, Single};
use crate::schema::users;
use diesel::prelude::*;

pub fn index(conn: &Conn) -> Multiple<User> {
    Ok(users::table.order(users::id.asc()).get_results(conn)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<User> {
    Ok(users::table.find(id).first(conn)?)
}

pub fn update(conn: &Conn, id: &i32, new_user: NewUser) -> Single<User> {
    let target = users::table.find(id);

    Ok(diesel::update(target).set(new_user).get_result(conn)?)
}

pub fn find_by_email(conn: &Conn, email: &str) -> Single<User> {
    Ok(users::table.filter(users::email.eq(email)).first(conn)?)
}
