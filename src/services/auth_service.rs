use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::user::{NewUser, User};
use crate::models::Single;
use crate::repositories::user_repository;
use argon2::{self, Config};
use std::env;

pub fn login(conn: &Conn, user: NewUser) -> Single<User> {
    let user_query = user_repository::find_by_email(conn, &user.email);

    match user_query {
        Ok(existing_user) => {
            if verify_hash(user.password.to_owned(), existing_user.password.to_owned()) {
                // let user_string: String = serde_json::to_string(&existing_user).unwrap();
                Ok(existing_user)
            } else {
                Err(ServiceError::Unauthorized)
            }
        }
        Err(_) => Err(ServiceError::Unauthorized),
    }
}

pub fn register(conn: &Conn, mut new_user: NewUser) -> Single<User> {
    new_user.password = create_hash(&new_user.password);
    Ok(user_repository::store(conn, new_user)?)
}

fn verify_hash(text: String, hash: String) -> bool {
    return argon2::verify_encoded(&hash, &text.into_bytes()).unwrap();
}

pub fn create_hash(text: &String) -> String {
    let text_to_hash = text.to_owned().into_bytes();
    let salt = env::var("SALT")
        .unwrap_or(String::from("Default Salt Value"))
        .into_bytes();

    let config = Config::default();
    argon2::hash_encoded(&text_to_hash, &salt, &config).unwrap()
}
