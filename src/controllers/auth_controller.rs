use super::AppData;
use super::GenericResponse;
use crate::models::user::{NewUser, User};
use crate::models::Single;
use actix_identity::Identity;
use actix_web::error::BlockingError;
use actix_web::{delete, post, web, HttpResponse, Responder};
use argon2::{self, Config};
use diesel::prelude::*;
use diesel::result::Error;
use std::env;

use crate::services::auth_service;

#[post("/login")]
pub async fn login(
    data: AppData,
    payload: web::Json<NewUser>,
    identity: Identity,
) -> GenericResponse {
    let data = data.lock().unwrap();
    let conn = &data.conn_pool.get().unwrap();
    let user = auth_service::login(conn, identity, payload.into())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/register")]
pub async fn register(app_data: AppData, payload: web::Json<NewUser>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let data = app_data.lock().unwrap();
    let conn = &data.conn_pool.get().unwrap();

    let hashed_password = create_hash(payload.password.to_owned());

    let insert_result: Result<User, Error> = diesel::insert_into(users)
        .values(NewUser {
            email: payload.email.to_owned(),
            password: hashed_password,
            display_name: None,
            profile_picture: None,
            display_picture: None,
            bio: None,
        })
        .get_result::<User>(conn);

    match insert_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
    }
}

#[delete("/logout")]
pub async fn logout(identity: Identity) -> impl Responder {
    identity.forget();
    HttpResponse::Ok().json("Logged Out")
}

pub fn create_hash(text: String) -> String {
    let text_to_hash = text.into_bytes();
    let salt = env::var("SALT")
        .unwrap_or(String::from("Default Salt Value"))
        .into_bytes();

    let config = Config::default();
    argon2::hash_encoded(&text_to_hash, &salt, &config).unwrap()
}

fn verify_hash(text: String, hash: String) -> bool {
    return argon2::verify_encoded(&hash, &text.into_bytes()).unwrap();
}
