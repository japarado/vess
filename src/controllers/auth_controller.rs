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

use crate::controllers::ok_closure;
use crate::errors::ServiceError;
use crate::services::auth_service;

#[post("/login")]
pub async fn login(
    data: AppData,
    payload: web::Json<NewUser>,
    identity: Identity,
) -> GenericResponse {
    web::block(move || -> Single<User> {
        let data = data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        let user = auth_service::login(conn, payload.into())?;
        Ok(user)
    })
    .await
    .map(|user| {
        let user_string = serde_json::to_string(&user).unwrap();
        identity.remember(user_string);
        ok_closure(user)
    })
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[post("/register")]
pub async fn register(app_data: AppData, payload: web::Json<NewUser>) -> impl Responder {
    web::block(move || -> Single<User> {
        let data = app_data.lock().unwrap();
        let conn = &data.conn_pool.get().unwrap();
        Ok(auth_service::register(conn, payload.into())?)
    })
    .await
    .map(|user| ok_closure(user))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
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
