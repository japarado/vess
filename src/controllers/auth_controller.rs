use crate::database::StatePool;
use actix_identity::Identity;
use actix_web::{delete, error::BlockingError, get, post, web, HttpResponse, Responder};

use crate::controllers::ok_response;
use crate::errors::ServiceError;
use crate::models::user::{AuthUser, NewUser, SingleUser, User};

use argon2::{self, Config};
use diesel::prelude::*;
use diesel::result::Error;
use std::env;

#[get("/me")]
pub async fn me(pool: StatePool, auth_user: AuthUser) -> Result<HttpResponse, ServiceError> {
    println!("{}", auth_user.id);
    web::block(move || -> SingleUser { Ok(User::show(pool, &auth_user.id)?) })
        .await
        .map(|post| ok_response(post))
        .map_err(|err| match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => ServiceError::InternalServerError,
        })
}

#[post("/login")]
pub async fn login(
    pool: StatePool,
    payload: web::Json<NewUser>,
    identity: Identity,
) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = &pool.get().unwrap();

    let query_result = users
        .filter(email.eq(payload.email.to_owned()))
        .first::<User>(conn);

    match query_result {
        Ok(user) => {
            let password_correct =
                verify_hash(payload.password.to_owned(), user.password.to_owned());
            if password_correct {
                let user_string: String = serde_json::to_string(&user).unwrap();
                println!("User String: {}", user_string);
                identity.remember(user_string);
                println!("Identity Value: {:?}", identity.identity());
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::Unauthorized().json("password incorrect")
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("email not found"),
    }
}

#[post("/register")]
pub async fn register(pool: StatePool, payload: web::Json<NewUser>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = &pool.get().unwrap();

    let hashed_password = create_hash(payload.password.to_owned());

    // let new_user = NewUser {
    //     email: payload.email.to_owned(),
    //     password: hashed_password,
    // };

    let insert_result: Result<User, Error> = diesel::insert_into(users)
        .values(NewUser {
            email: payload.email.to_owned(),
            password: hashed_password,
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

fn create_hash(text: String) -> String {
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
