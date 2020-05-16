extern crate frank_jwt;
#[macro_use]
extern crate diesel;

use crate::errors::ServiceError;
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    get, middleware::Logger, middleware::NormalizePath, web, App, HttpResponse, HttpServer,
    Responder,
};

use dotenv::dotenv;
use std::env;

mod controllers;
mod database;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schema;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = database::create_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::config)
            .service(index)
            .default_service(web::route().to(fallback_route))
            // .wrap(middleware::say_hi_middleware::SayHi)
            // .wrap(middleware::auth_middleware::Auth)
            .wrap(Cors::new().supports_credentials().max_age(3600).finish())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(
                    env::var("SECRET_KEY")
                        .unwrap_or("actix web secret key".to_string())
                        .as_bytes(),
                )
                .name("auth-cookie")
                .domain(env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()))
                // .max_age(86400)
                .max_age(0)
                .secure(false),
            ))
            .wrap(NormalizePath)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[get("/")]
pub async fn index() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json("Main API Page"))
}

pub async fn fallback_route() -> impl Responder {
    HttpResponse::NotFound().json("Not Found")
}
