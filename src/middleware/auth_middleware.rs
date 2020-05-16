use std::pin::Pin;
use std::task::{Context, Poll};

use crate::errors::ServiceError;
use crate::models::user::{NewUser, User};
use actix_http::cookie::Cookie;
use actix_identity::Identity;
use actix_service::{Service, Transform};
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, http::header::HeaderValue, Error, HttpRequest,
};
use futures::future::{ok, Either, Ready};
use futures::Future;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let cookies = req.headers().get("cookie");

        // match cookies {
        //     Some(cookie_map) => {
        //         match  Cookie::parse_encoded(cookie_map.to_str().unwrap()) {
        //             Ok(auth_cookie_pair) => {
        //                 println!("Cookie Name: {}", auth_cookie_pair.name());
        //                 println!("Cookie Value: {}", auth_cookie_pair.value());
        //                 let user: NewUser = serde_json::from_str(auth_cookie_pair.value()).unwrap();
        //                 println!("{:?}", user);
        //             }
        //             Err(_) => {
        //                 println!("No cookie named \"auth-cookie\"");
        //             }
        //         }
        //     },
        //     None => {
        //         println!("No cookie header found!");
        //     }
        // };

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}
