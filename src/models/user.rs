use crate::errors::ServiceError;
use crate::schema::users;
use actix_identity::Identity;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::Future;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Serialize, Deserialize, Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip)]
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

pub type AuthUser = User;

impl FromRequest for User {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = Identity::from_request(req, payload);

        Box::pin(async move {
            if let Some(identity) = fut.await?.identity() {
                let user: Self = serde_json::from_str(&identity)?;
                return Ok(user);
            }
            Err(Error::from(ServiceError::Unauthorized))
        })
    }
}
