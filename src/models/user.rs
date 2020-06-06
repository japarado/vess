use crate::errors::ServiceError;
use crate::schema::users;
use actix_identity::Identity;
use actix_web::web::Json;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::Future;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Serialize, Deserialize, Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub display_picture: Option<String>,
    pub bio: Option<String>,
    #[serde(skip)]
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub display_picture: Option<String>,
    pub bio: Option<String>,
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

impl From<Json<NewUser>> for NewUser {
    fn from(new_user_json: Json<NewUser>) -> Self {
        Self {
            email: new_user_json.email.clone(),
            display_name: new_user_json.display_name.clone(),
            profile_picture: new_user_json.profile_picture.clone(),
            display_picture: new_user_json.display_picture.clone(),
            bio: new_user_json.bio.clone(),
            password: new_user_json.password.clone(),
        }
    }
}

impl From<User> for NewUser {
    fn from(user: User) -> Self {
        Self {
            email: user.email,
            password: user.password,
            display_name: user.display_name,
            display_picture: user.display_picture,
            profile_picture: user.profile_picture,
            bio: user.bio,
        }
    }
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub password: String,
    pub confirmation: String,
}

impl From<Json<ResetPasswordRequest>> for ResetPasswordRequest {
    fn from(reset_password_json: Json<ResetPasswordRequest>) -> Self {
        Self {
            password: reset_password_json.password.to_owned(),
            confirmation: reset_password_json.confirmation.to_owned(),
        }
    }
}
