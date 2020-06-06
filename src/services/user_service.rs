use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::user::{NewUser, User};
use crate::models::{Multiple, Single};
use crate::repositories::user_repository;

pub fn index(conn: &Conn) -> Multiple<User> {
    Ok(user_repository::index(conn)?)
}

pub fn show(conn: &Conn, id: &i32) -> Single<User> {
    Ok(user_repository::show(conn, id)?)
}

pub fn update(conn: &Conn, id: &i32, new_user: NewUser) -> Single<User> {
    let user = user_repository::show(conn, id)?;
    if &user.id == id {
        // Create a new instance of NewUser as we do not want to edit the email and password
        let updated_user = NewUser {
            email: user.email,
            password: user.password,
            ..new_user
        };

        Ok(user_repository::update(conn, id, updated_user)?)
    } else {
        Err(ServiceError::Unauthorized)
    }
}
