use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::user::{NewUser, ResetPasswordRequest, User};
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
        // Create a new instance of NewUser as we do not want to alter the email and password
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

pub fn reset_password(conn: &Conn, id: &i32, payload: ResetPasswordRequest) -> Single<User> {
    let mut user = user_repository::show(conn, id)?;

    if &user.id == id {
        if payload.password == payload.confirmation {
            user.password = payload.password;
            Ok(user_repository::update(conn, id, user.into())?)
        } else {
            Err(ServiceError::Conflict("Passwords do not match".to_string()))
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}
